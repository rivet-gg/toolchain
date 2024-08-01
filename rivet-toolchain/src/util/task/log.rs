use serde::Serialize;
use tokio::{
	fs::File,
	io::{AsyncWriteExt, BufWriter},
	sync::mpsc,
};

#[derive(Serialize)]
pub enum LogEvent {
	Stdout(String),
	Stderr(String),
}

pub async fn log_writer(mut log_rx: mpsc::UnboundedReceiver<LogEvent>, log_file: Option<File>) {
	if let Some(log_file) = log_file {
		// Write to file

		let mut log_writer = BufWriter::new(log_file);

		while let Some(event) = log_rx.recv().await {
			// HACK: serde_json::to_writer is not async
			let event_json = match serde_json::to_vec(&event) {
				Ok(x) => x,
				Err(err) => {
					eprintln!("Failed to serialize event: {err:?}");
					continue;
				}
			};
			if log_writer.write_all(&event_json).await.is_err() {
				eprintln!("Failed to write event to stdout log file");
			}
			if log_writer.write_all(b"\n").await.is_err() {
				eprintln!("Failed to write newline to stdout log file");
			}
			if log_writer.flush().await.is_err() {
				eprintln!("Failed to flush stdout log file");
			}
		}
	} else {
		// Write to stdout

		while let Some(event) = log_rx.recv().await {
			match event {
				LogEvent::Stdout(x) => println!("{x}"),
				LogEvent::Stderr(x) => eprintln!("{x}"),
			}
		}
	}
}
