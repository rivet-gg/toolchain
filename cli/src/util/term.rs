use anyhow::Result;
use console::{style, StyledObject, Term};
use std::str::FromStr;
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

	// pub fn warn(msg: impl Display, data: impl Display) {
	// 	eprintln!("{} {}", style(msg).bold().yellow(), data);
	// }

	pub fn error(msg: impl Display, data: impl Display) {
		eprintln!("{} {}", style(msg).bold().red(), data);
	}
}

pub struct Prompt {
	message: String,
	context: Option<String>,
	docs: Option<String>,
	docs_url: Option<String>,
	default_value: Option<String>,
	indent: usize,
}

impl Prompt {
	pub fn new(message: impl ToString) -> Prompt {
		Prompt {
			message: message.to_string(),
			context: None,
			docs: None,
			docs_url: None,
			default_value: None,
			indent: 0,
		}
	}

	pub fn context(mut self, context: impl ToString) -> Self {
		self.context = Some(context.to_string());
		self
	}

	pub fn docs(mut self, docs: impl ToString) -> Self {
		self.docs = Some(docs.to_string());
		self
	}

	pub fn docs_url(mut self, docs_url: impl ToString) -> Self {
		self.docs_url = Some(docs_url.to_string());
		self
	}

	pub fn default_value(mut self, default_value: impl ToString) -> Self {
		self.default_value = Some(default_value.to_string());
		self
	}

	pub fn indent(mut self, indent: usize) -> Self {
		self.indent = indent;
		self
	}
}

impl Prompt {
	fn gen_indent(&self) -> String {
		"    ".repeat(self.indent)
	}

	fn print_header(&self) {
		let i = self.gen_indent();

		eprintln!();
		eprint!("{i}");
		if let Some(context) = &self.context {
			eprint!("{} ", style(format!("[{context}]")).bold());
		}
		eprintln!("{}", style(&self.message).bold().blue());
		if let Some(docs) = &self.docs {
			eprintln!("{i}  {}", style(&docs).italic());
		}
		if let Some(docs_url) = &self.docs_url {
			eprintln!("{i}  {}", style(&docs_url).italic().underlined().cyan());
		}
		if let Some(default_value) = &self.default_value {
			eprintln!(
				"{i}  {} {}",
				style("Defaults to").italic(),
				style(&default_value).italic().bold()
			);
		}
	}

	async fn read_line(&self, term: &Term) -> Result<String> {
		self.read_line_inner(term, false).await
	}

	// async fn read_line_secure(&self, term: &Term) -> Result<String> {
	// 	self.read_line_inner(term, true).await
	// }

	async fn read_line_inner(&self, term: &Term, secure: bool) -> Result<String> {
		term.flush()?;

		let input = if secure {
			tokio::task::block_in_place(|| term.read_secure_line())?
		} else {
			tokio::task::block_in_place(|| term.read_line())?
		};

		let input_trimmed = input.trim();

		if input_trimmed.is_empty() {
			if let Some(default_value) = self.default_value.as_ref() {
				return Ok(default_value.clone());
			}
		}

		Ok(input_trimmed.to_string())
	}

	pub async fn bool(&self, term: &Term) -> Result<bool> {
		let i = self.gen_indent();

		self.print_header();

		loop {
			eprint!("{i}  {}", style("[y/n] ").bold());
			let input = self.read_line(term).await?;

			match input.to_lowercase().as_str() {
				"y" | "yes" | "t" | "true" => return Ok(true),
				"n" | "no" | "f" | "false" => return Ok(false),
				_ => {
					status::error(format!("{i}  Invalid bool"), "Must be y or n");
				}
			}
		}
	}

	pub async fn parsed<T>(&self, term: &Term) -> Result<T>
	where
		T: FromStr,
	{
		let i = self.gen_indent();

		self.print_header();

		loop {
			eprint!("{i}  {} ", style(">").bold());
			let input = self.read_line(term).await?;

			if let Ok(parsed) = input.parse::<T>() {
				return Ok(parsed);
			} else {
				status::error(format!("{i}  Invalid input"), "");
			}
		}
	}

	pub async fn string(&self, term: &Term) -> Result<String> {
		let i = self.gen_indent();

		self.print_header();

		loop {
			eprint!("{i}  {} ", style(">").bold());
			let input = self.read_line(term).await?;

			if !input.is_empty() {
				return Ok(input);
			} else {
				status::error(format!("{i}  Empty input"), "");
			}
		}
	}

	// pub async fn string_secure(&self, term: &Term) -> Result<String> {
	// 	let i = self.gen_indent();

	// 	self.print_header();

	// 	loop {
	// 		eprint!("{i}  {} ", style("[input hidden]").bold());
	// 		let input = self.read_line_secure(term).await?;

	// 		if !input.is_empty() {
	// 			return Ok(input);
	// 		} else {
	// 			status::error(format!("{i}  Empty input"), "");
	// 		}
	// 	}
	// }
}
