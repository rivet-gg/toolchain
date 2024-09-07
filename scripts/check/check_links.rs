#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! anyhow = "*"
//! walkdir = "2.3.2"
//! regex = "1.5.4"
//! reqwest = { version = "0.11.6", features = ["json"] }
//! tokio = { version = "1.14.0", features = ["full"] }
//! ```

use std::fs::read_to_string;
use walkdir::WalkDir;
use regex::Regex;
use reqwest::StatusCode;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<()> {
    // Defines a regex to find URLs
    let url_re = Regex::new(r"https?://[\w/\.\-_]+")?;
    
    walk_dir(&url_re, "cli").await?;
    walk_dir(&url_re, "core").await?;

    Ok(())
}

async fn walk_dir(url_re: &Regex, dir: &str) -> Result<()> {
    // Walks through current directory and its subdirectories
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "rs" ||  ext == "yaml" || ext == "toml" || ext == "json")) {

        println!("> {}", entry.path().display());
        
        // Reads the file to a string
        let contents = read_to_string(entry.path())?;
        
        // Finds all URLs in the file
        for url in url_re.find_iter(&contents) {
                // Exclude URLs
                if url.as_str().contains("api.rivet.gg") {
                    continue;
                }

            // Sends a GET request to each URL and prints the status
            let url_str = url.as_str();
            let status = check_link(url_str).await?;
            let emoji = if status.is_success() { "🟢" } else { "🔴" };
            println!("  {emoji} {url_str}: {status}");
        }
    }

    Ok(())
}

async fn check_link(url: &str) -> Result<StatusCode> {
    let client = reqwest::Client::new();
    let response = client.get(url).timeout(Duration::from_secs(5)).send().await?;
    Ok(response.status())
}

