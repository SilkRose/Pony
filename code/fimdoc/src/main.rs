use camino::Utf8Path;
use markdown::mdast::{BlockQuote, Heading, Node, Paragraph, Root};
use markdown::{to_mdast, ParseOptions};
use std::env;
use std::fs;

enum ErrorType {
	Warn,
	Fail,
}

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
		const WARN: ErrorType = ErrorType::Warn;
		let text = handle_node(&tokens, WARN);
		println!("{text}");
	} else {
		eprintln!("File not found!");
	}
}

fn handle_node(node: &Node, warn: ErrorType) -> String {
	node.children()
		.unwrap()
		.iter()
		.map(|n| md_to_bbcode(n, &warn).unwrap())
		.collect::<Vec<_>>()
		.join("\n")
}

fn md_to_bbcode(node: &Node, warn: &ErrorType) -> Option<String> {
	match node {
		markdown::mdast::Node::Root(root) => Some(handle_root(root, warn)),
		markdown::mdast::Node::BlockQuote(quote) => Some(handle_quote(quote, warn)),
		markdown::mdast::Node::FootnoteDefinition(_) => todo!(),
		markdown::mdast::Node::MdxJsxFlowElement(_) => warn_error("MdxJsFlowElement", &warn),
		markdown::mdast::Node::List(_) => todo!(),
		markdown::mdast::Node::MdxjsEsm(_) => warn_error("MdxjsEsm", &warn),
		markdown::mdast::Node::Toml(_) => warn_error("Toml", &warn),
		markdown::mdast::Node::Yaml(_) => warn_error("Yaml", &warn),
		markdown::mdast::Node::Break(_) => Some("\n".into()),
		markdown::mdast::Node::InlineCode(_) => todo!(),
		markdown::mdast::Node::InlineMath(_) => todo!(),
		markdown::mdast::Node::Delete(_) => todo!(),
		markdown::mdast::Node::Emphasis(_) => todo!(),
		markdown::mdast::Node::MdxTextExpression(_) => warn_error("MdxTextExpression", &warn),
		markdown::mdast::Node::FootnoteReference(_) => todo!(),
		markdown::mdast::Node::Html(_) => warn_error("HTML", &warn),
		markdown::mdast::Node::Image(_) => todo!(),
		markdown::mdast::Node::ImageReference(_) => todo!(),
		markdown::mdast::Node::MdxJsxTextElement(_) => warn_error("MdxJsxTextElement", &warn),
		markdown::mdast::Node::Link(_) => todo!(),
		markdown::mdast::Node::LinkReference(_) => todo!(),
		markdown::mdast::Node::Strong(_) => todo!(),
		markdown::mdast::Node::Text(text) => Some(text.value.clone()),
		markdown::mdast::Node::Code(_) => todo!(),
		markdown::mdast::Node::Math(_) => todo!(),
		markdown::mdast::Node::MdxFlowExpression(_) => warn_error("MdxFlowExpression", &warn),
		markdown::mdast::Node::Heading(heading) => Some(handle_heading(heading, warn)),
		markdown::mdast::Node::Table(_) => warn_error("Table", &warn),
		markdown::mdast::Node::ThematicBreak(_) => Some("[hr]".into()),
		markdown::mdast::Node::TableRow(_) => warn_error("TableRow", &warn),
		markdown::mdast::Node::TableCell(_) => warn_error("TableCell", &warn),
		markdown::mdast::Node::ListItem(_) => todo!(),
		markdown::mdast::Node::Definition(_) => todo!(),
		markdown::mdast::Node::Paragraph(paragraph) => Some(handle_paragraph(paragraph, warn)),
	}
}

fn warn_error(token: &str, error: &ErrorType) -> Option<String> {
	match error {
		ErrorType::Warn => {
			eprintln!("WARNING: unsupported syntax skipped: {}", token);
			None
		}
		ErrorType::Fail => {
			panic!("WARNING: unsupported syntax found: {}", token)
		}
	}
}

fn handle_root(root: &Root, warn: &ErrorType) -> String {
	root.children
		.iter()
		.map(|node| md_to_bbcode(node, warn).unwrap())
		.collect::<Vec<_>>()
		.join("")
}

fn handle_quote(blockquote: &BlockQuote, warn: &ErrorType) -> String {
	let quote = blockquote
		.children
		.iter()
		.map(|quote| md_to_bbcode(quote, warn).unwrap())
		.collect::<Vec<_>>()
		.join("");
	format!("[quote]{quote}[/quote]\n")
}

fn handle_heading(heading: &Heading, warn: &ErrorType) -> String {
	let text = heading
		.children
		.iter()
		.map(|h| md_to_bbcode(h, warn).unwrap())
		.collect::<Vec<_>>()
		.join("");
	format!("[h{l}]{text}[/h{l}]\n", l = heading.depth)
}

fn handle_paragraph(paragraph: &Paragraph, warn: &ErrorType) -> String {
	paragraph
		.children
		.iter()
		.map(|p| md_to_bbcode(p, warn).unwrap())
		.collect::<Vec<_>>()
		.join("\n")
}
