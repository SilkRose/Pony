use atty::Stream;
use std::io;

pub fn get_stdin() -> Option<String> {
	if atty::isnt(Stream::Stdin) {
		Some(
			io::stdin()
				.lines()
				.map(|l| l.unwrap())
				.collect::<Vec<_>>()
				.join("\n"),
		)
	} else {
		None
	}
}
