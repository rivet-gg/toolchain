pub struct TaskConfig {
	pub prefer_multithreaded: bool,
}

impl TaskConfig {
	pub const fn default_const() -> Self {
		Self {
			prefer_multithreaded: false,
		}
	}
}

pub trait Task {
	type Input: DeserializeOwned;
	type Output: Serialize;

	const CONFIG: TaskConfig = TaskConfig::default_const();

	fn name() -> &'static str;
	fn run<OnEvent>(
		task: TaskCtx,
		input: Self::Input,
		on_event: OnEvent,
	) -> impl Future<Output = GlobalResult<Self::Output>>
	where
		OnEvent: Fn(output::OutputEvent);
}

pub struct RunTaskJsonOutput {
	pub success: bool,
}
