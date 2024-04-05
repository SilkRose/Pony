use atty::Stream;
use std::io;

pub fn get_stdin() -> Option<String> {
	atty::isnt(Stream::Stdin).then(|| {
		io::stdin()
			.lines()
			.map(|l| l.unwrap())
			.collect::<Vec<_>>()
			.join("\n")
	})
}
