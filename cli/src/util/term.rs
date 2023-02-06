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

pub fn link(msg: impl ToString) -> StyledObject<String> {
	style(msg.to_string()).italic().underlined()
}

pub mod status {
	use console::style;
	use std::fmt::Display;

	pub fn info(msg: impl Display, data: impl Display) {
		eprintln!("{} {}", style(msg).bold().blue(), data);
	}

	// pub fn progress(msg: impl Display, data: impl Display) {
	// 	eprintln!("{} {}", style(msg).bold().green(), data);
	// }

	pub fn success(msg: impl Display, data: impl Display) {
		eprintln!("{} {}", style(msg).bold().green(), data);
	}

	pub fn warn(msg: impl Display, data: impl Display) {
		eprintln!("{} {}", style(msg).bold().yellow(), data);
	}

	pub fn error(msg: impl Display, data: impl Display) {
		eprintln!("{} {}", style(msg).bold().red(), data);
	}
}

pub mod input {
	use anyhow::Result;
	use console::{style, Term};
	use std::fmt::Display;

	pub async fn string(term: &Term, msg: impl Display + Clone) -> Result<String> {
		loop {
			eprint!("{} ", style(msg.clone()).bold().blue());
			term.flush()?;
			let input = tokio::task::block_in_place(|| term.read_line())?;

			if input.len() > 0 {
				return Ok(input);
			} else {
				super::status::error("Empty entry", "");
				eprintln!();
				continue;
			}
		}
	}

	pub async fn string_with_tip(
		term: &Term,
		msg: impl Display + Clone,
		tip: impl Display + Clone,
	) -> Result<String> {
		loop {
			eprint!(
				"{} {} ",
				style(msg.clone()).bold().blue(),
				style(format!("({tip})")).italic()
			);
			term.flush()?;
			let input = tokio::task::block_in_place(|| term.read_line())?;

			if input.len() > 0 {
				return Ok(input);
			} else {
				super::status::error("Empty entry", "");
				eprintln!();
				continue;
			}
		}
	}

	pub async fn secure(term: &Term, msg: impl Display) -> Result<String> {
		eprint!("{} ", style(msg).bold().blue());
		term.flush()?;
		let input = tokio::task::block_in_place(|| term.read_secure_line())?;
		Ok(input)
	}

	pub async fn secure_with_docs(
		term: &Term,
		msg: impl Display,
		docs: impl Display + Clone,
		url: impl Display + Clone,
	) -> Result<String> {
		eprint!(
			"{} (secure input)\n  {}\n  {}\n  ",
			style(msg).bold().blue(),
			style(docs).italic(),
			style(url).underlined().cyan()
		);
		term.flush()?;
		let input = tokio::task::block_in_place(|| term.read_secure_line())?;
		Ok(input)
	}

	pub async fn bool(term: &Term, msg: impl Display + Clone) -> Result<bool> {
		loop {
			eprint!(
				"{} {} ",
				style(msg.clone()).bold().blue(),
				style("(y/n)").italic()
			);
			term.flush()?;
			let input = tokio::task::block_in_place(|| term.read_char())?;
			eprintln!();

			match input {
				'y' | 'Y' => return Ok(true),
				'n' | 'N' => return Ok(false),
				_ => {
					super::status::error("Invalid Bool", "Must be y or n");
					eprintln!();
					continue;
				}
			}
		}
	}

	pub async fn bool_with_docs(
		term: &Term,
		msg: impl Display + Clone,
		docs: impl Display + Clone,
		url: impl Display + Clone,
	) -> Result<bool> {
		loop {
			eprint!(
				"{}\n  {}\n  {}\n  {} ",
				style(msg.clone()).bold().blue(),
				style(docs.clone()).italic(),
				style(url.clone()).italic().underlined().cyan(),
				style("(y/n)").bold()
			);
			term.flush()?;
			let input = tokio::task::block_in_place(|| term.read_char())?;
			eprintln!();

			match input {
				'y' | 'Y' => return Ok(true),
				'n' | 'N' => return Ok(false),
				_ => {
					super::status::error("Invalid Bool", "Must be y or n");
					eprintln!();
					continue;
				}
			}
		}
	}
}
