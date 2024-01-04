use console::{style, Color, Style, Term};
use regex::Regex;

pub fn center_text(input: &str, width: usize) -> String {
	let longest_len = input
		.lines()
		.map(|line| length_without_color_codes(line))
		.max()
		.unwrap_or(0);

	let padding = (width.saturating_sub(longest_len)) / 2;
	let padding_str = " ".repeat(padding);

	input
		.split('\n')
		.map(|line| format!("{padding_str}{line}"))
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn rainbow_line(len: usize) {
	let term = Term::stdout();
	let colors = [
		Color::Red,
		Color::Yellow,
		Color::Green,
		Color::Cyan,
		Color::Blue,
		Color::Magenta,
	];
	let block_len = len / colors.len();
	let block = "█".repeat(block_len);

	// Print rainbow lines
	for &color in colors.iter() {
		let styled = Style::new().fg(color).apply_to(&block);
		term.write_str(&styled.to_string()).unwrap();
	}

	// Fill the remaining blocks
	let remaining = len % (colors.len() * block_len);
	for _ in 0..remaining {
		let styled = Style::new()
			.fg(colors.last().unwrap().clone())
			.apply_to("█");
		term.write_str(&styled.to_string()).unwrap();
	}

	term.write_str("\n").unwrap();
}

pub fn render_box(input: &str, width: usize) -> String {
	let mut output = String::new();
	// Render top
	output.push_str("┌");
	output.push_str(&"─".repeat(width - 2));
	output.push_str("┐\n");
	// Render middle
	for line in input.split('\n') {
		output.push_str("│");
		let centered_line = center_text(line, width - 2);
		output.push_str(&centered_line);
		output.push_str(&" ".repeat((width - 2) - length_without_color_codes(&centered_line)));
		output.push_str("│\n");
	}
	// Render bottom
	output.push_str("└");
	output.push_str(&"─".repeat(width - 2));
	output.push_str("┘\n");
	output
}

pub fn render_box_padded(input: &str, padding: usize) -> String {
	// Add 2 to account for the border
	let width = input
		.split('\n')
		.map(|x| length_without_color_codes(x))
		.max()
		.unwrap_or(2)
		+ (padding * 2)
		+ 2;
	render_box(input, width)
}

// We need to be able to center text with color codes, but we don't want to
// include the color codes in the length calculation. This function removes
// color codes from a string and returns the length.
fn length_without_color_codes(input: &str) -> usize {
	let re = Regex::new("\x1B\\[[^m]*m").unwrap();
	re.replace_all(input, "").chars().count()
}
