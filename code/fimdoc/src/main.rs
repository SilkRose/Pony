use camino::Utf8Path;
use fimdoc::parser::{parse, WarningType};
use std::env;
use std::fs;

fn main() {
	let filename = &env::args().collect::<Vec<_>>()[1];
	if !filename.ends_with(".md") {
		panic!("File must be Markdown.")
	};
	let filepath = Utf8Path::new(filename);
	if Utf8Path::exists(filepath) {
		let md = fs::read_to_string(filepath).unwrap();
		let warn = WarningType::Warn;
		let bbcode = parse(md, &warn);
		println!("{bbcode}");
	} else {
		eprintln!("File not found!");
	}
}
