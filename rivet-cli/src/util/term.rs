use std::time::Duration;

use console::{style, StyledObject};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tabled::{Table, Tabled};

pub fn table<T>(iter: impl IntoIterator<Item = T>)
where
	T: Tabled,
{
	let mut table = Table::new(iter).with(tabled::Style::rounded());
	if let Some((w, _)) = term_size::dimensions() {
		table = table.with(tabled::Width::wrap(w));
	}
	println!("{}", table);
}

pub fn link(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).italic().underlined()
}

#[derive(Clone)]
pub enum EitherProgressBar {
	Single(ProgressBar),
	Multi(MultiProgress),
}

// Must be enabled with `pb.set_draw_target(ProgressDrawTarget::stderr()))`
pub fn progress_bar() -> ProgressBar {
	let pb = ProgressBar::hidden();
	pb.enable_steady_tick(Duration::from_millis(250));
	pb
}

pub fn pb_style_file() -> ProgressStyle {
	ProgressStyle::default_bar()
		.progress_chars("=> ")
		.template(&format!(
			"{{spinner:.dim}} {{elapsed:.bold}} {}{{eta:.dim}}{} [{{bar:23}}] {{percent:.bold}}{} {}{{bytes:.dim}}{}{{total_bytes:.dim}}{} {{binary_bytes_per_sec:.dim}}{} {{wide_msg}}",
			style("(T-").dim(),
			style(")").dim(),
			style("%").bold(),
			style("(").dim(),
			style("/").dim(),
			style(",").dim(),
			style(")").dim(),
		))
		.expect("invalid progress bar style")
}

pub fn pb_style_error() -> ProgressStyle {
	ProgressStyle::default_bar()
		.template(&format!("{} {{wide_msg:.red}}", style("!").bold().red()))
		.expect("invalid progress bar style")
}
