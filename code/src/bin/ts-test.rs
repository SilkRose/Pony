use camino::Utf8Path;
use pinkie_pie::stderr::{print_error, ErrColor};
use pinkie_pie::stdin::get_stdin;
use std::process::exit;
use std::{env, fs};
use twilight_sparkle::parser::parse;
use twilight_sparkle::text_stats::word_count;

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
	let args: Vec<String> = env::args().skip(1).collect();
	let (input, output) = match (&stdin, args.len()) {
		(Some(_), 0) => (Input::Stdin, Output::Stdout),
		(Some(_), 1) => (Input::Stdin, Output::File),
		(None, 2) => (Input::File, Output::File),
		(None, 1) => (Input::File, Output::Stdout),
		(Some(_), 2..) | (None, 3..) => {
			print_error("Too many arguments provided!", ErrColor::Red);
			exit(1);
		}
		(None, 0) => {
			print_error("Not enough arguments and no stdin found!", ErrColor::Red);
			exit(1);
		}
	};
	let md = match input {
		Input::Stdin => stdin.unwrap(),
		Input::File => {
			let filename = match output {
				Output::Stdout => &args[args.len() - 1],
				Output::File => &args[args.len() - 2],
			};
			if !filename.ends_with(".md") {
				print_error("Input file must be Markdown.", ErrColor::Red);
				exit(1);
			};
			let filepath = Utf8Path::new(filename);
			if Utf8Path::exists(filepath) {
				fs::read_to_string(filepath).unwrap()
			} else {
				print_error("File not found!", ErrColor::Red);
				exit(1);
			}
		}
	};
	let bbcode = parse(md.clone());
	match output {
		Output::Stdout => println!("{bbcode}"),
		Output::File => {
			let filename = &args[args.len() - 1];
			let filepath = Utf8Path::new(&filename);
			if !Utf8Path::exists(filepath) {
				fs::write(filepath, bbcode).unwrap()
			} else {
				print_error("File already exists!", ErrColor::Red);
				exit(1);
			}
		}
	}
	println!("{}", word_count(md));
}
