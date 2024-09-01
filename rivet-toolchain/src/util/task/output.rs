use serde::Serialize;
use std::{
	fs::File,
	io::{self, BufWriter, Write},
};
use tokio::{sync::mpsc, task::block_in_place};

use super::OutputStyle;

#[derive(Serialize)]
pub enum OutputEvent {
	#[serde(rename = "stdout")]
	Stdout(String),
	#[serde(rename = "stderr")]
	Stderr(String),
	#[serde(rename = "output")]
	Output {
		result: Box<serde_json::value::RawValue>,
	},
}

impl OutputEvent {
	fn print(&self, output_style: OutputStyle, writer: &mut impl Write) {
		match output_style {
			OutputStyle::Json => {
				if let Err(err) = serde_json::to_writer(&mut *writer, self) {
					eprintln!("failed to serialize output: {err:?}");
				}
				writeln!(writer).unwrap();
			}
			OutputStyle::Plain => match self {
				OutputEvent::Stdout(x) => eprintln!("{x}"),
				OutputEvent::Stderr(x) => eprintln!("{x}"),
				OutputEvent::Output { result } => {
					if let Err(err) =
						writeln!(writer, "{}", serde_json::to_string(&result).unwrap())
					{
						eprintln!("failed to serialize output: {err:?}");
					}
				}
			},
		}
	}
}

pub async fn output_writer(
	mut output_rx: mpsc::UnboundedReceiver<OutputEvent>,
	output_file: Option<File>,
	output_style: OutputStyle,
) {
	if let Some(output_file) = output_file {
		// Write to file
		let mut output_writer = BufWriter::new(output_file);

		while let Some(event) = output_rx.recv().await {
			block_in_place(|| {
				event.print(output_style, &mut output_writer);
			});
			if let Err(err) = block_in_place(|| output_writer.flush()) {
				eprintln!("Failed to flush stdout output file: {err:?}");
			}
		}
	} else {
		// Write to stdout
		let mut stdout = io::stdout();
		while let Some(event) = output_rx.recv().await {
			block_in_place(|| {
				event.print(output_style, &mut stdout);
			});
		}
	}
}
