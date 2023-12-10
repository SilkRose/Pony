use atty::Stream;
use camino::Utf8Path;
use fimdoc::parser::{parse, WarningType};
use std::{env, fs, io};

enum Input {
	Stdin,
	File,
}

enum Output {
	Stdout,
	File,
}

fn main() {
	let stdin = get_stdin();
	let args: Vec<String> = env::args().collect();
	let (input, output) = match (&stdin, args.len() - 1) {
		(Some(_), 0) => (Input::Stdin, Output::Stdout),
		(Some(_), 1) => (Input::Stdin, Output::File),
		(Some(_), _) => panic!("Too many arguments provided!"),
		(None, 2) => (Input::File, Output::File),
		(None, 1) => (Input::File, Output::Stdout),
		(None, _) => panic!("Not enough arguments and no stdin found!"),
	};
	let md = match input {
		Input::Stdin => stdin.unwrap(),
		Input::File => {
			let filename = &args[1];
			if !filename.ends_with(".md") {
				panic!("File must be Markdown.")
			};
			let filepath = Utf8Path::new(filename);
			if Utf8Path::exists(filepath) {
				fs::read_to_string(filepath).unwrap()
			} else {
				panic!("File not found!")
			}
		}
	};
	let warn = WarningType::Warn;
	let bbcode = parse(md, &warn);
	match output {
		Output::Stdout => println!("{bbcode}"),
		Output::File => {
			let filename = &args[args.len() - 1];
			let filepath = Utf8Path::new(&filename);
			if !Utf8Path::exists(filepath) {
				fs::write(filepath, bbcode).unwrap()
			} else {
				panic!("File already exists!")
			}
		}
	}
}

fn get_stdin() -> Option<String> {
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
