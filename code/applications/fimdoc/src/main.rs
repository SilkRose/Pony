use camino::Utf8Path;
use golden_oak::md_to_bbcode::{parse, WarningType};
use golden_oak::stderr::{print_error, ErrColor};
use golden_oak::stdin::get_stdin;
use std::process::exit;
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
	let warn = parse_input(&args);
	let (input, output) = match (&stdin, args.len()) {
		(Some(_), 0) => (Input::Stdin, Output::Stdout),
		(Some(_), 1) => (Input::Stdin, Output::File),
		(Some(_), _) => {
			print_error("Too many arguments provided!", ErrColor::Red);
			exit(1);
		}
		(None, 2) => (Input::File, Output::File),
		(None, 1) => (Input::File, Output::Stdout),
		(None, _) => {
			print_error("Not enough arguments and no stdin found!", ErrColor::Red);
			exit(1);
		}
	};
	let md = match input {
		Input::Stdin => stdin.unwrap(),
		Input::File => {
			let filename = &args[1];
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
	let bbcode = parse(md, &warn);
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
}

fn parse_input(args: &Vec<String>) -> WarningType {
	if args.len() == 1 {
		return WarningType::Warn;
	}
	match args[1].as_str() {
		"-w" | "--warn" => WarningType::Warn,
		"-f" | "--fail" => WarningType::Fail,
		"-q" | "--quiet" => WarningType::Quiet,
		"-h" | "--help" => {
			print_help();
			exit(0);
		}
		"-v" | "--version" => {
			println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
			exit(0);
		}
		_ => WarningType::Warn,
	}
}

fn print_help() {
	println!(
		"{} {}

{}

Usage: fimdoc [OPTIONS] [INPUT] [OUTPUT]

Arguments:
  [INPUT]...   Input Markdown file, must end in .md
  [OUTPUT]...  Output text file

Options:
  -h, --help           Print help
  -V, --version        Print version",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		env!("CARGO_PKG_DESCRIPTION")
	)
}
