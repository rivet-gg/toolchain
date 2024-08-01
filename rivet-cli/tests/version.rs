use serde_json::json;
use uuid::Uuid;

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
	let _user_config = rivet_cli::commands::config::read_config(
		vec![
			("cdn.site_id".into(), json!(Uuid::new_v4())),
			("matchmaker.docker.image_id".into(), json!(Uuid::new_v4())),
		],
		Some("my-ns"),
	)
	.await
	.unwrap();
}
