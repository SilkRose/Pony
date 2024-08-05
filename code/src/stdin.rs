use super::command::execute_command;
use std::io::{self, IsTerminal, Read, Write};
use std::{error::Error, fs};

/// Function to collect stdin or return None.
pub fn get_stdin() -> Option<String> {
	let stdin = io::stdin();
	let mut handle = stdin.lock();
	if stdin.is_terminal() {
		let mut buffer = String::new();
		handle.read_to_string(&mut buffer).ok()?;
		Some(buffer)
	} else {
		None
	}
}

pub fn ask<F>(question: &str, filter: Option<(F, &str)>) -> Result<String, Box<dyn Error>>
where
	F: Fn(&str) -> bool,
{
	let mut answer = String::new();
	println!("{}", question);
	io::stdout().flush()?;
	io::stdin().read_line(&mut answer)?;
	if let Some((filter_fn, filter_err)) = filter {
		if !filter_fn(answer.trim()) {
			eprintln!("{filter_err}");
			return ask(question, Some((filter_fn, filter_err)));
		}
	}
	Ok(answer.trim().to_string())
}

pub fn ask_longform(question: &str, temp_filename: &str) -> Result<String, Box<dyn Error>> {
	fs::File::create_new(temp_filename)?;
	fs::write(temp_filename, question)?;
	let mut stdout = io::stdout();
	stdout.write_all(b"\x1B[?1049h")?;
	stdout.flush()?;
	execute_command(&format!("vim {temp_filename}"))?;
	stdout.write_all(b"\x1B[?1049l")?;
	stdout.flush()?;
	let answer = fs::read_to_string(temp_filename)?;
	fs::remove_file(temp_filename)?;
	Ok(answer)
}
