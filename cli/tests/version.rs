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
	let _user_config = rivet::commands::version::read_user_config(
		vec![
			("cdn.site".into(), json!("xxxx")),
			("matchmaker.docker.image".into(), json!("xxxx")),
		],
		Some("my-ns"),
	)
	.await
	.unwrap();
}
