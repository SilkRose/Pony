use atty::Stream;
use std::error::Error;
use std::io::{self, Write};

/// Function to collect stdin or return None.
pub fn get_stdin() -> Option<String> {
	atty::isnt(Stream::Stdin).then(|| {
		io::stdin()
			.lines()
			.map(|l| l.unwrap())
			.collect::<Vec<_>>()
			.join("\n")
	})
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
