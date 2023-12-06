use camino::Utf8Path;
use markdown::tokenize;
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
		let tokens = tokenize(&md);
		println!("{md}");
		tokens.iter().for_each(|t| match t {
			markdown::Block::Header(text, hl) => {
				println!("{text:?}, {hl}")
			}
			markdown::Block::Paragraph(text) => {
				println!("{text:?}")
			}
			markdown::Block::Blockquote(_) => todo!(),
			markdown::Block::CodeBlock(_, _) => todo!(),
			markdown::Block::OrderedList(_, _) => todo!(),
			markdown::Block::UnorderedList(_) => todo!(),
			markdown::Block::Raw(text) => {
				println!("{text}")
			}
			markdown::Block::Hr => todo!(),
		})
	} else {
		eprintln!("File not found!");
	}
}
