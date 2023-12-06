use camino::Utf8Path;
use markdown::{to_mdast, ParseOptions};
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
		let options = ParseOptions::default();
		let tokens = to_mdast(&md, &options).unwrap();
		println!("{md}");
		println!("{tokens:?}");
		tokens.children().unwrap().iter().for_each(|t| match t {
			markdown::mdast::Node::Root(_) => todo!(),
			markdown::mdast::Node::BlockQuote(_) => todo!(),
			markdown::mdast::Node::FootnoteDefinition(_) => todo!(),
			markdown::mdast::Node::MdxJsxFlowElement(_) => todo!(),
			markdown::mdast::Node::List(_) => todo!(),
			markdown::mdast::Node::MdxjsEsm(_) => todo!(),
			markdown::mdast::Node::Toml(_) => todo!(),
			markdown::mdast::Node::Yaml(_) => todo!(),
			markdown::mdast::Node::Break(_) => todo!(),
			markdown::mdast::Node::InlineCode(_) => todo!(),
			markdown::mdast::Node::InlineMath(_) => todo!(),
			markdown::mdast::Node::Delete(_) => todo!(),
			markdown::mdast::Node::Emphasis(_) => todo!(),
			markdown::mdast::Node::MdxTextExpression(_) => todo!(),
			markdown::mdast::Node::FootnoteReference(_) => todo!(),
			markdown::mdast::Node::Html(_) => todo!(),
			markdown::mdast::Node::Image(_) => todo!(),
			markdown::mdast::Node::ImageReference(_) => todo!(),
			markdown::mdast::Node::MdxJsxTextElement(_) => todo!(),
			markdown::mdast::Node::Link(_) => todo!(),
			markdown::mdast::Node::LinkReference(_) => todo!(),
			markdown::mdast::Node::Strong(_) => todo!(),
			markdown::mdast::Node::Text(_) => todo!(),
			markdown::mdast::Node::Code(_) => todo!(),
			markdown::mdast::Node::Math(_) => todo!(),
			markdown::mdast::Node::MdxFlowExpression(_) => todo!(),
			markdown::mdast::Node::Heading(t) => {
				println!("{:?}, {:?}", t.children[0], t.depth);
			},
			markdown::mdast::Node::Table(_) => todo!(),
			markdown::mdast::Node::ThematicBreak(_) => todo!(),
			markdown::mdast::Node::TableRow(_) => todo!(),
			markdown::mdast::Node::TableCell(_) => todo!(),
			markdown::mdast::Node::ListItem(_) => todo!(),
			markdown::mdast::Node::Definition(_) => todo!(),
			markdown::mdast::Node::Paragraph(t) => {
				println!("{t:?}");
			}
		})
	} else {
		eprintln!("File not found!");
	}
}
