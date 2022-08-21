use console::{style, StyledObject};
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

pub fn link<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).italic().underlined()
}

pub mod status {
	use console::style;
	use std::fmt::Display;

	pub fn info(msg: impl Display, data: impl Display) {
		eprintln!("    {} {}", style(msg).bold().blue(), data);
	}

	pub fn progress(msg: impl Display, data: impl Display) {
		eprintln!("    {} {}", style(msg).bold().green(), data);
	}

	pub fn success(msg: impl Display, data: impl Display) {
		eprintln!("    {} {}", style(msg).bold().green(), data);
	}

	pub fn warn(msg: impl Display, data: impl Display) {
		eprintln!("    {} {}", style(msg).bold().yellow(), data);
	}

	pub fn error(msg: impl Display, data: impl Display) {
		eprintln!("    {} {}", style(msg).bold().red(), data);
	}
}
