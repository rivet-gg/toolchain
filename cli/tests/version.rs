use serde_json::json;

#[tokio::test]
async fn basic() {
	std::env::set_current_dir(
		std::env::current_dir()
			.unwrap()
			.join("tests")
			.join("static")
			.join("basic"),
	)
	.unwrap();
	let _user_config = rivet_cli::commands::version::read_config(
		vec![
			("cdn.site_id".into(), json!("xxxx")),
			("matchmaker.docker.image_id".into(), json!("xxxx")),
		],
		Some("my-ns"),
	)
	.await
	.unwrap();
}
