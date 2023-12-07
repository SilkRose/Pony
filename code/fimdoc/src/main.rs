use camino::Utf8Path;
use markdown::mdast::{BlockQuote, Node, Root};
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
		let options = ParseOptions::gfm();
		let tokens = to_mdast(&md, &options).unwrap();
		println!("{md}");
		println!("{tokens:?}");
		let text = md_to_bbcode(&tokens);
		println!("{text}");
	} else {
		eprintln!("File not found!");
	}
}

fn md_to_bbcode(node: &Node) -> String {
	node.children()
		.unwrap()
		.iter()
		.map(|n| match n {
			markdown::mdast::Node::Root(root) => handle_root(root).join("\n"),
			markdown::mdast::Node::BlockQuote(quote) => {
				format!("[quote]{}[/quote]\n\n", handle_quote(quote).join("\n"))
			}
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
			markdown::mdast::Node::Heading(h) => {
				format!("{:?}, {:?}", h.children[0], h.depth)
			}
			markdown::mdast::Node::Table(_) => todo!(),
			markdown::mdast::Node::ThematicBreak(_) => todo!(),
			markdown::mdast::Node::TableRow(_) => todo!(),
			markdown::mdast::Node::TableCell(_) => todo!(),
			markdown::mdast::Node::ListItem(_) => todo!(),
			markdown::mdast::Node::Definition(_) => todo!(),
			markdown::mdast::Node::Paragraph(p) => {
				format!("{p:?}")
			}
		})
		.collect::<Vec<_>>().join("\n")
}

fn handle_root(root: &Root) -> Vec<String> {
	root.children.iter().map(|node| md_to_bbcode(node)).collect()
}

fn handle_quote(blockquote: &BlockQuote) -> Vec<String> {
	blockquote.children.iter().map(|quote| md_to_bbcode(quote)).collect()
}
