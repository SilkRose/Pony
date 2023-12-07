use camino::Utf8Path;
use markdown::mdast::{BlockQuote, Heading, Node, Paragraph};
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
		Node::Root(root) => Some(handle_child_nodes(&root.children, warn, "")),
		Node::BlockQuote(quote) => Some(handle_quote(quote, warn)),
		Node::FootnoteDefinition(_) => todo!(),
		Node::MdxJsxFlowElement(_) => warn_error("MdxJsFlowElement", &warn),
		Node::List(_) => todo!(),
		Node::MdxjsEsm(_) => warn_error("MdxjsEsm", &warn),
		Node::Toml(_) => warn_error("Toml", &warn),
		Node::Yaml(_) => warn_error("Yaml", &warn),
		Node::Break(_) => Some("\n".into()),
		Node::InlineCode(_) => todo!(),
		Node::InlineMath(_) => todo!(),
		Node::Delete(_) => todo!(),
		Node::Emphasis(_) => todo!(),
		Node::MdxTextExpression(_) => warn_error("MdxTextExpression", &warn),
		Node::FootnoteReference(_) => todo!(),
		Node::Html(_) => warn_error("HTML", &warn),
		Node::Image(_) => todo!(),
		Node::ImageReference(_) => todo!(),
		Node::MdxJsxTextElement(_) => warn_error("MdxJsxTextElement", &warn),
		Node::Link(_) => todo!(),
		Node::LinkReference(_) => todo!(),
		Node::Strong(_) => todo!(),
		Node::Text(text) => Some(text.value.clone()),
		Node::Code(_) => todo!(),
		Node::Math(_) => todo!(),
		Node::MdxFlowExpression(_) => warn_error("MdxFlowExpression", &warn),
		Node::Heading(heading) => Some(handle_heading(heading, warn)),
		Node::Table(_) => warn_error("Table", &warn),
		Node::ThematicBreak(_) => Some("[hr]".into()),
		Node::TableRow(_) => warn_error("TableRow", &warn),
		Node::TableCell(_) => warn_error("TableCell", &warn),
		Node::ListItem(_) => todo!(),
		Node::Definition(_) => todo!(),
		Node::Paragraph(paragraph) => Some(handle_paragraph(paragraph, warn)),
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

fn handle_child_nodes(nodes: &[Node], warn: &ErrorType, separator: &str) -> String {
	nodes
		.iter()
		.map(|node| md_to_bbcode(node, warn).unwrap())
		.collect::<Vec<_>>()
		.join(separator)
}

fn handle_quote(blockquote: &BlockQuote, warn: &ErrorType) -> String {
	let quote = handle_child_nodes(&blockquote.children, warn, "");
	format!("[quote]{quote}[/quote]\n")
}

fn handle_heading(heading: &Heading, warn: &ErrorType) -> String {
	let text = handle_child_nodes(&heading.children, warn, "");
	format!("[h{l}]{text}[/h{l}]\n", l = heading.depth)
}

fn handle_paragraph(paragraph: &Paragraph, warn: &ErrorType) -> String {
	handle_child_nodes(&paragraph.children, warn, "\n")
}
