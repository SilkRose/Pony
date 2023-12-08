use camino::Utf8Path;
use markdown::mdast::{BlockQuote, Heading, Node, Paragraph, Root};
use markdown::{to_mdast, ParseOptions};
use std::env;
use std::fs;

enum WarningType {
	_Fail,
	_Silent,
	Warn,
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
		let warn = WarningType::Warn;
		let text = handle_node(&tokens, warn);
		println!("{text}");
	} else {
		eprintln!("File not found!");
	}
}

fn handle_node(node: &Node, warn: WarningType) -> String {
	node.children()
		.unwrap()
		.iter()
		.map(|n| md_to_bbcode(n, &warn).unwrap())
		.collect::<Vec<_>>()
		.join("\n")
}

fn md_to_bbcode(node: &Node, warn: &WarningType) -> Option<String> {
	match node {
		Node::Root(root) => Some(handle_root(root, warn)),
		Node::BlockQuote(quote) => Some(handle_quote(quote, warn)),
		Node::FootnoteDefinition(_) => handle_warning("FootnoteDefinition", warn),
		Node::MdxJsxFlowElement(_) => handle_warning("MdxJsFlowElement", warn),
		Node::List(_) => todo!(),
		Node::MdxjsEsm(_) => handle_warning("MdxjsEsm", warn),
		Node::Toml(_) => handle_warning("Toml", warn),
		Node::Yaml(_) => handle_warning("Yaml", warn),
		Node::Break(_) => Some(handle_break()),
		Node::InlineCode(_) => todo!(),
		Node::InlineMath(_) => todo!(),
		Node::Delete(_) => todo!(),
		Node::Emphasis(_) => todo!(),
		Node::MdxTextExpression(_) => handle_warning("MdxTextExpression", warn),
		Node::FootnoteReference(_) => handle_warning("FootnoteReference", warn),
		Node::Html(_) => handle_warning("HTML", warn),
		Node::Image(_) => todo!(),
		Node::ImageReference(_) => todo!(),
		Node::MdxJsxTextElement(_) => handle_warning("MdxJsxTextElement", warn),
		Node::Link(_) => todo!(),
		Node::LinkReference(_) => todo!(),
		Node::Strong(_) => todo!(),
		Node::Text(text) => Some(text.value.clone()),
		Node::Code(_) => todo!(),
		Node::Math(_) => todo!(),
		Node::MdxFlowExpression(_) => handle_warning("MdxFlowExpression", warn),
		Node::Heading(heading) => Some(handle_heading(heading, warn)),
		Node::Table(_) => handle_warning("Table", warn),
		Node::ThematicBreak(_) => Some(handle_thematic_break()),
		Node::TableRow(_) => handle_warning("TableRow", warn),
		Node::TableCell(_) => handle_warning("TableCell", warn),
		Node::ListItem(_) => todo!(),
		Node::Definition(_) => todo!(),
		Node::Paragraph(paragraph) => Some(handle_paragraph(paragraph, warn)),
	}
}

fn handle_warning(token: &str, error: &WarningType) -> Option<String> {
	match error {
		WarningType::_Fail => {
			panic!("WARNING: unsupported syntax found: {}", token)
		}
		WarningType::_Silent => None,
		WarningType::Warn => {
			eprintln!("WARNING: unsupported syntax skipped: {}", token);
			None
		}
	}
}

fn handle_child_nodes(nodes: &[Node], warn: &WarningType, separator: &str) -> String {
	nodes
		.iter()
		.map(|node| md_to_bbcode(node, warn).unwrap())
		.collect::<Vec<_>>()
		.join(separator)
}

fn handle_root(root: &Root, warn: &WarningType) -> String {
	handle_child_nodes(&root.children, warn, "")
}

fn handle_quote(blockquote: &BlockQuote, warn: &WarningType) -> String {
	let quote = handle_child_nodes(&blockquote.children, warn, "");
	format!("[quote]{quote}[/quote]\n\n")
}

fn handle_break() -> String {
	"[hr]".into()
}

fn handle_heading(heading: &Heading, warn: &WarningType) -> String {
	let text = handle_child_nodes(&heading.children, warn, "");
	format!("[h{l}]{text}[/h{l}]\n", l = heading.depth)
}

fn handle_thematic_break() -> String {
	"[hr]".into()
}

fn handle_paragraph(paragraph: &Paragraph, warn: &WarningType) -> String {
	handle_child_nodes(&paragraph.children, warn, "\n")
}
