use camino::Utf8Path;
use fimdoc::parser::{parse, WarningType};
use indoc::printdoc;
use pinkie_pie::stderr::{print_error, ErrColor};
use pinkie_pie::stdin::get_stdin;
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
	let args: Vec<String> = env::args().skip(1).collect();
	let (warn, length) = parse_input(&args);
	let (input, output) = match (&stdin, length) {
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

fn parse_input(args: &[String]) -> (WarningType, usize) {
	if args.is_empty() {
		return (WarningType::Warn, 0);
	}
	match args[0].as_str() {
		"-w" | "--warn" => (WarningType::Warn, args[1..].len()),
		"-f" | "--fail" => (WarningType::Fail, args[1..].len()),
		"-q" | "--quiet" => (WarningType::Quiet, args[1..].len()),
		"-h" | "--help" => {
			print_help();
			exit(0);
		}
		"-v" | "--version" => {
			println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
			exit(0);
		}
		_ => {
			if args[0].starts_with("-") {
				print_error("Incorrect argument option!", ErrColor::Red);
				print_help();
				exit(1);
			} else {
				(WarningType::Warn, args.len())
			}
		}
	}
}

fn print_help() {
	printdoc! {"
		{} {}

		{}

		Usage Examples:
		  fimdoc input.md output.txt
		  fimdoc -q input.md | bbcode
		  md | fimdoc | bbcode
		  md | fimdoc --fail output.txt

		Options:
		  -w, --warn         Warns on unsupported markdown syntax
		  -f, --fail         Fails on unsupported markdown syntax
		  -q, --quiet        Skips over unsupported markdown syntax
		  -h, --help         Print help
		  -v, --version      Print version\n",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		env!("CARGO_PKG_DESCRIPTION")
	}
}
