use std::process::exit;
use terminal_color_builder::OutputFormatter as tcb;

pub enum ErrColor {
	Yellow,
	Red,
}

pub fn print_error(message: String, color: ErrColor, quit: bool) {
	match color {
		ErrColor::Yellow => tcb::new().fg().yellow(),
		ErrColor::Red => tcb::new().fg().red(),
	}
	.text(message)
	.print();
	if quit {
		exit(1);
	}
}
