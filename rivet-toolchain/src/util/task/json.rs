/// Used to run tasks with raw input/output string. This is useful for binding tasks to non-Rust
/// environments, such as raw dylibs or odd engines.
macro_rules! gen_run_task {
    ( $( $task:ty ),* $(,)? ) => {
        pub async fn run_task_json(run_config: RunConfig, name: &str, input_json: &str) -> RunTaskJsonOutput {
            $(
                if name == <$task>::name() {
                    let input = serde_json::from_str::<<$task as Task>::Input>(&input_json)
                        .expect("deserialize task input");
                    let output = run_task::<$task>(run_config, input).await;
                    let success = output.is_ok();
                    return RunTaskJsonOutput {
                        success,
                    };
                }
            )*

            panic!("unknown task {name}")
        }

        pub fn get_task_config(name: &str) -> &'static TaskConfig {
            $(
                if name == <$task>::name() {
                    return &<$task>::CONFIG;
                }
            )*

            panic!("unknown task {name}")
        }
    };
}
