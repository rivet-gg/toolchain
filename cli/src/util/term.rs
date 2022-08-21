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

pub fn label(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()).blue())
}

pub fn label_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).blue()
}

pub fn info(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()))
}

pub fn info_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string())
}

pub fn link(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()).italic().underlined())
}

pub fn link_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).italic().underlined()
}

pub fn success(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()).bold().green());
}

pub fn success_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).bold().green()
}

pub fn warn(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()).bold().yellow());
}

pub fn warn_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).bold().yellow()
}

pub fn error(msg: impl AsRef<str>) {
	eprintln!("{}", style(msg.as_ref()).bold().red());
}

pub fn error_fmt<'a>(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).red()
}
