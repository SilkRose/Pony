use terminal_color_builder::OutputFormatter as tcb;

pub enum ErrColor {
	Yellow,
	Red,
}

pub fn print_error(message: &str, color: ErrColor) {
	let error = match color {
		ErrColor::Yellow => tcb::new().fg().yellow(),
		ErrColor::Red => tcb::new().fg().red(),
	}
	.text(message.to_string())
	.print();
	println!("{error}");
}
