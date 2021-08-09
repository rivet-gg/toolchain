use anyhow::*;
use async_recursion::async_recursion;
use clap::Clap;
use std::{
    env,
    path::{Path, PathBuf},
    time::Instant,
};
use tokio::fs;

const GAME_IMAGE_TAG: &'static str = "rivet-game:latest";

#[derive(Clap)]
#[clap()]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,

    #[clap(long)]
    base_path: Option<String>,
}

#[derive(Clap)]
enum SubCommand {
    Auth(AuthSubCommand),
    Build(BuildSubCommand),
    Site(SiteSubCommand),
}

#[derive(Clap)]
enum AuthSubCommand {
    Token(AuthTokenOpts),
}

#[derive(Clap)]
struct AuthTokenOpts {}

#[derive(Clap)]
enum BuildSubCommand {
    Push(BuildPushOpts),
}

#[derive(Clap)]
struct BuildPushOpts {
    #[clap(index(1))]
    tag: String,

    #[clap(long)]
    name: Option<String>,
}

#[derive(Clap)]
enum SiteSubCommand {
    Push(SitePushOptions),
}

#[derive(Clap)]
struct SitePushOptions {
    #[clap(index(1))]
    path: String,

    #[clap(long)]
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    // Read config
    let config_path = home::home_dir()
        .context("missing home dir")?
        .join(".config")
        .join("rivetctl.json");
    let config = match fs::read(&config_path).await {
        Ok(buf) => serde_json::from_slice::<rivetctl::config::Config>(buf.as_slice())?,
        Err(_) => rivetctl::config::Config::default(),
    };

    // Build ctx
    let ctx = rivetctl::ctx::SharedCtx::new(config.clone(), opts.base_path.clone()).await?;
    match opts.subcmd {
        SubCommand::Auth(auth_cmd) => match auth_cmd {
            AuthSubCommand::Token(_) => {
                print!("Auth token: ");

                // Read token from stdin
                let token = tokio::task::spawn_blocking(|| {
                    use std::io::BufRead;

                    let stdin = std::io::stdin();
                    let mut iterator = stdin.lock().lines();
                    iterator.next().unwrap().context("token not provided")
                })
                .await??;

                // Create new config
                let mut new_config = config.clone();
                new_config.auth.token = Some(token.trim().to_owned());

                // Create new context to check the token
                let new_ctx =
                    rivetctl::ctx::SharedCtx::new(new_config.clone(), opts.base_path.clone())
                        .await?;
                let inspect = rivetctl::apis::auth_api::inspect(&new_ctx.api_config()?).await?;
                println!("{:?}", inspect);

                // Save new config
                write_config(&new_config, &config_path).await?;
            }
        },
        SubCommand::Build(build_cmd) => match build_cmd {
            BuildSubCommand::Push(push_opts) => {
                let api_config = ctx.api_config()?;

                let game_id = infer_game_id(&api_config).await?;

                let tmp_image_file = tempfile::NamedTempFile::new()?;
                let tmp_path = tmp_image_file.into_temp_path();

                // Archive the image
                println!("\n\n> Archiving image");
                let tag_cmd = tokio::process::Command::new("docker")
                    .arg("image")
                    .arg("tag")
                    .arg(&push_opts.tag)
                    .arg(GAME_IMAGE_TAG)
                    .output()
                    .await?;
                ensure!(tag_cmd.status.success(), "failed to archive docker image");

                let save_cmd = tokio::process::Command::new("docker")
                    .arg("image")
                    .arg("save")
                    .arg("--output")
                    .arg(&tmp_path)
                    .arg(GAME_IMAGE_TAG)
                    .output()
                    .await?;
                ensure!(save_cmd.status.success(), "failed to archive docker image");

                // Inspect the image
                let image_file_meta = fs::metadata(&tmp_path).await?;

                // Create build
                let display_name = push_opts
                    .name
                    .clone()
                    .unwrap_or_else(|| push_opts.tag.clone());
                let content_type = "application/x-tar";
                println!("\n\n> Creating build \"{}\"", display_name);
                let build_res = rivetctl::apis::game_api::create_game_build(
                    &api_config,
                    &game_id,
                    rivetctl::models::InlineObject6 {
                        display_name,
                        image_file: Box::new(rivetctl::models::UploadPrepareFile {
                            path: "image.tar".into(),
                            content_type: Some(content_type.into()),
                            content_length: image_file_meta.len() as i32,
                        }),
                    },
                )
                .await?;

                println!(
                    "\n\n> Uploading ({len:.1} MB)",
                    len = (image_file_meta.len() as f64 / 1024. / 1024.)
                );
                upload_file(
                    &api_config.client,
                    &build_res.image_presigned_request,
                    tmp_path,
                    Some(content_type),
                )
                .await?;

                println!("\n\n> Completing");
                rivetctl::apis::upload_api::complete_upload(
                    &api_config,
                    &build_res.upload_id,
                    serde_json::json!({}),
                )
                .await?;
            }
        },
        SubCommand::Site(cdn_cmd) => match cdn_cmd {
            SiteSubCommand::Push(push_opts) => {
                let api_config = ctx.api_config()?;

                let game_id = infer_game_id(&api_config).await?;

                let upload_path = env::current_dir()?.join(push_opts.path);
                println!("Upload path: {}", upload_path.display());

                // Index the directory
                let mut files = Vec::new();
                prepare_upload_dir(&upload_path, &upload_path, &mut files).await?;
                let total_len = files
                    .iter()
                    .fold(0, |acc, x| acc + x.prepared.content_length);
                println!(
                    "Found {count} files ({len:.1} MB)",
                    count = files.len(),
                    len = (total_len as f64 / 1024. / 1024.)
                );

                // Create site
                let display_name = push_opts.name.clone().unwrap_or_else(|| {
                    upload_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map(str::to_owned)
                        .unwrap_or_else(|| "Site".to_owned())
                });
                println!("\n\n> Creating site \"{}\"", display_name);
                let site_res = rivetctl::apis::game_api::create_game_cdn_site(
                    &api_config,
                    &game_id,
                    rivetctl::models::InlineObject5 {
                        display_name,
                        files: files.iter().map(|f| f.prepared.clone()).collect(),
                    },
                )
                .await?;

                println!("\n\n> Uploading");
                for presigned_req in site_res.presigned_requests {
                    // Find the matching prepared file
                    let file = files
                        .iter()
                        .find(|f| f.prepared.path == presigned_req.path)
                        .context("missing prepared file")?;

                    upload_file(
                        &api_config.client,
                        &presigned_req,
                        &file.absolute_path,
                        file.prepared.content_type.as_ref(),
                    )
                    .await?;
                }

                println!("\n\n> Completing");
                rivetctl::apis::upload_api::complete_upload(
                    &api_config,
                    &site_res.upload_id,
                    serde_json::json!({}),
                )
                .await?;
            }
        },
    }

    Ok(())
}

/// Writes a modified config to the file system.
async fn write_config(config: &rivetctl::config::Config, path: &Path) -> Result<()> {
    // Create parent directory
    fs::create_dir_all(&path.parent().context("config path parent")?).await?;

    // Write config
    let config_str = serde_json::to_string(config)?;
    fs::write(path, config_str).await?;

    Ok(())
}

/// Prepared file that will be uploaded to S3.
struct UploadFile {
    absolute_path: PathBuf,
    prepared: rivetctl::models::UploadPrepareFile,
}

/// Lists all files in a directory and returns the data required to upload them.
#[async_recursion]
async fn prepare_upload_dir(
    base_path: &Path,
    path: &Path,
    files: &mut Vec<UploadFile>,
) -> Result<()> {
    use std::path::Component;

    let mut dir = fs::read_dir(path).await?;
    while let Some(file) = dir.next_entry().await? {
        let file_path = file.path();
        let file_meta = file.metadata().await?;

        // Upload dir
        if file_meta.is_dir() {
            prepare_upload_dir(base_path, &file_path, files).await?;
        }

        // Index file
        if file_meta.is_file() {
            // Read relative path component and change to Unix-style line endings
            let path = file_path
                .strip_prefix(base_path)?
                .components()
                .filter_map(|c| match c {
                    Component::Normal(name) => name.to_str().map(str::to_string),
                    _ => None,
                })
                .collect::<Vec<String>>()
                .join("/");

            // Attempt to guess the MIME type
            let content_type = mime_guess::from_path(&file_path)
                .first_raw()
                .map(str::to_string);

            files.push(UploadFile {
                absolute_path: file_path.clone(),
                prepared: rivetctl::models::UploadPrepareFile {
                    path,
                    content_type,
                    content_length: file_meta.len() as i32,
                },
            });
        }
    }

    Ok(())
}

/// Uploads a file to a given URL.
async fn upload_file(
    client: &reqwest::Client,
    presigned_req: &rivetctl::models::UploadPresignedRequest,
    path: impl AsRef<Path>,
    content_type: Option<impl ToString>,
) -> Result<()> {
    use tokio::fs::File;
    use tokio_util::codec::{BytesCodec, FramedRead};

    let content_type = content_type.map(|x| x.to_string());

    // Read file
    let file = File::open(path.as_ref()).await?;
    let file_meta = file.metadata().await?;

    println!(
        "  * {path} -> {url} [{size:.1} MB] [{mime}]",
        path = presigned_req.path,
        url = presigned_req.url,
        size = (file_meta.len() as f64 / 1024. / 1024.),
        mime = content_type.clone().unwrap_or_default(),
    );

    // Create body
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = reqwest::Body::wrap_stream(stream);

    // Upload file
    let start = Instant::now();
    let mut req = client
        .put(&presigned_req.url)
        .header("content-length", file_meta.len());
    if let Some(content_type) = content_type {
        req = req.header("content-type", content_type.to_string());
    }
    let res = req.body(body).send().await?;
    ensure!(
        res.status().is_success(),
        "failed to upload file: {}\n{:?}",
        res.status(),
        res.text().await
    );

    let upload_time = start.elapsed();
    println!("    Finished in {:.3}s", upload_time.as_secs_f64());

    Ok(())
}

/// Uses the provided token to find the game ID to modify.
async fn infer_game_id(
    api_config: &rivetctl::apis::configuration::Configuration,
) -> Result<String> {
    let inspect = rivetctl::apis::auth_api::inspect(&api_config).await?;
    let game_cloud = inspect.agent.game_cloud.context("invalid token agent")?;

    Ok(game_cloud.game_id)
}
