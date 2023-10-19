use assert_cmd::Command;

#[tokio::test]
async fn e2e_deploy() {
	let project_dir = std::env::current_dir()
		.unwrap()
		.join("tests")
		.join("static")
		.join("basic");

	let rivet_api_endpoint = std::env::var("RIVET_API_ENDPOINT").expect("RIVET_API_ENDPOINT");
	let rivet_token = std::env::var("RIVET_TOKEN").expect("RIVET_TOKEN");

	let mut cmd = Command::cargo_bin("rivet-cli").unwrap();
	let assert = cmd
		.current_dir(project_dir)
		.env("RIVET_API_ENDPOINT", rivet_api_endpoint)
		.env("RIVET_TOKEN", rivet_token)
		.arg("deploy")
		.arg("-n")
		.arg("staging")
		.assert()
		.success();
}
