use assert_cmd::Command;

#[tokio::test]
#[ignore]
async fn e2e_deploy() {
	let project_dir = std::env::current_dir()
		.unwrap()
		.join("tests")
		.join("static")
		.join("basic");

	let mut cmd = Command::cargo_bin("rivet-cli").unwrap();
	let _assert = cmd
		.current_dir(project_dir)
		.arg("deploy")
		.arg("-n")
		.arg("staging")
		.assert()
		.success();
}
