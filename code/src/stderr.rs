use std::process::exit;

pub fn print_warning(warning: &str) {
	eprintln!("\x1b[93m{warning}\x1b[0m"); // Yellow for warnings
}

pub fn print_error(error: &str) -> ! {
	eprintln!("\x1b[91m{error}\x1b[0m"); // Red for errors
	exit(0)
}
