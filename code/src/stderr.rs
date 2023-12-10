use terminal_color_builder::OutputFormatter as tcb;

pub enum ErrColor {
	Yellow,
	Red,
}

pub fn print_error(message: String, color: ErrColor) {
	match color {
		ErrColor::Yellow => tcb::new().fg().yellow(),
		ErrColor::Red => tcb::new().fg().red(),
	}
	.text(message)
	.print();
}
