use camino::Utf8Path;
use clap::{Parser, ValueEnum};
use indoc::formatdoc;
use pony::md_to_bbcode::{parse, WarningType};
use pony::md_to_plaintext;
use pony::stderr::{print_error, ErrColor};
use pony::stdin::get_stdin;
use std::process::exit;
use std::{env, fs};

#[derive(Debug, Parser)]
#[command(name = "FimDoc",
bin_name = env!("CARGO_BIN_NAME"),
	version,
	about = formatdoc!("
		Convert markdown into FIMFiction BBCode or plaintext.

		example: fimdoc -i input.md -o output.txt
		example: md | fimdoc -w quiet output.txt"))
]

struct Args {
	#[arg(short, long, value_enum, default_value_t = Format::BBCode)]
	/// Output format
	format: Format,
	#[arg(short, long, value_enum, default_value_t = WarningType::Warn)]
	/// Warning level
	warning: WarningType,
	#[arg(short, long)]
	/// Input file path [reads from stdin if not provided]
	input: Option<String>,
	#[arg(short, long)]
	/// Output file path [prints to stdout if not provided]
	output: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Format {
	/// FIMFiction BBCode syntax
	BBCode,
	/// Syntax removed plaintext
	Plaintext,
}

fn main() {
	let args = Args::parse();
	let stdin = get_stdin();
	if stdin.is_some() && args.input.is_some() {
		print_error("File input and stdin both found!", ErrColor::Red);
		exit(1);
	}
	let md = match args.input {
		None => stdin.unwrap(),
		Some(filename) => {
			if !filename.ends_with(".md") {
				print_error("Input file must be Markdown.", ErrColor::Red);
				exit(1);
			};
			let filepath = Utf8Path::new(&filename);
			if Utf8Path::exists(filepath) {
				fs::read_to_string(filepath).unwrap()
			} else {
				print_error("File not found!", ErrColor::Red);
				exit(1);
			}
		}
	};
	let output = match args.format {
		Format::BBCode => parse(md, &args.warning),
		Format::Plaintext => md_to_plaintext::parse(md),
	};
	match args.output {
		None => println!("{output}"),
		Some(filename) => {
			let filepath = Utf8Path::new(&filename);
			if !Utf8Path::exists(filepath) {
				fs::write(filepath, output).unwrap()
			} else {
				print_error("File already exists!", ErrColor::Red);
				exit(1);
			}
		}
	}
}
