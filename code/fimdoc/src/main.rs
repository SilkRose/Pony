use camino::Utf8Path;
use fimdoc::parser::{parse, WarningType};
use rarity::stdin::get_stdin;
use std::{env, fs};

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
	let (warn, args) = match args[1].as_str() {
		"-w" | "--warn" => (WarningType::Warn, args[2..].to_owned()),
		"-f" | "--fail" => (WarningType::Fail, args[2..].to_owned()),
		"-q" | "--quiet" => (WarningType::Quiet, args[2..].to_owned()),
		_ => (WarningType::Warn, args[1..].to_owned()),
	};
	let (input, output) = match (&stdin, args.len()) {
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
