use terminal_color_builder::OutputFormatter as tcb;

/// Enum for error color.
pub enum ErrColor {
	/// Yellow error message.
	Yellow,
	/// Red error message.
	Red,
}

/// Function to print and error in a specified color.
pub fn print_error(message: &str, color: ErrColor) {
	let error = match color {
		ErrColor::Yellow => tcb::new().fg().yellow(),
		ErrColor::Red => tcb::new().fg().red(),
	}
	.text(message.to_string())
	.print();
	println!("{error}");
}
