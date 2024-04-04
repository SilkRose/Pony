use camino::Utf8Path;
use golden_oak_library::md_to_bbcode::{parse, WarningType};
use golden_oak_library::stderr::{print_error, ErrColor};
use golden_oak_library::stdin::get_stdin;
use std::process::exit;
use std::{env, fs};
use indoc::printdoc;

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
		(Some(_), 1) => (Input::Stdin, Output::Stdout),
		(Some(_), 2) => (Input::Stdin, Output::File),
		(Some(_), _) => {
			print_error("Too many arguments provided!", ErrColor::Red);
			exit(1);
		}
		(None, 3) => (Input::File, Output::File),
		(None, 2) => (Input::File, Output::Stdout),
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

fn parse_input(args: &[String]) -> WarningType {
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
	printdoc! {"
		{} {}

		{}

		Usage Examples:
		fimdoc input.md output.txt
		fimdoc -q input.md | bbcode
		md | fimdoc | bbcode
		md | fimdoc --fail output.txt

		Options:
		  -h, --help           Print help
		  -v, --version        Print version\n",
		env!("CARGO_PKG_NAME"),
		env!("CARGO_PKG_VERSION"),
		env!("CARGO_PKG_DESCRIPTION")
	}
}
