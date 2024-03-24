use camino::Utf8Path;
use markdown::mdast::{
	BlockQuote, Code, Definition, Delete, Emphasis, Heading, Image, ImageReference, InlineCode,
	InlineMath, Link, LinkReference, List, ListItem, Math, Node, Paragraph, Root, Strong,
};
use markdown::{to_mdast, ParseOptions};
use pony::fs::find_files_in_dir;
use pony::stderr::{print_error, ErrColor};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Definitions = HashMap<String, String>;

fn main() {
	if Utf8Path::new("./publish").is_dir() {
		fs::remove_dir_all("./publish").unwrap()
	}
	fs::create_dir("./publish").unwrap();
	let includes = Some(vec![
		Regex::new(r"stories|flash-fiction").unwrap(),
		Regex::new(r".md$").unwrap(),
	]);
	let excludes = Some(vec![
		Regex::new(r"archive").unwrap(),
		Regex::new(r"ideas.md$|names.md$|README.md$").unwrap(),
	]);
	find_files_in_dir("../", true, &includes, &excludes)
		.par_iter()
		.for_each(|input| {
			let md = fs::read_to_string(input).unwrap();
			let bbcode = parse(md);
			let output = input.replace("../", "./publish/").replace(".md", ".txt");
			fs::create_dir_all(Utf8Path::new(&output).parent().unwrap()).unwrap();
			fs::write(output, bbcode).unwrap();
			println!("Converted: {input}");
		});
}

fn parse(md: String) -> String {
	let node = to_mdast(&md, &ParseOptions::gfm()).unwrap();
	handle_node(&node)
}

fn handle_node(node: &Node) -> String {
	let definitions = node
		.children()
		.unwrap()
		.iter()
		.filter_map(|n| match n {
			Node::Definition(definition) => Some(handle_definition(definition)),
			_ => None,
		})
		.collect::<Definitions>();

	node.children()
		.unwrap()
		.iter()
		.filter_map(|n| md_to_bbcode(n, &definitions))
		.collect::<Vec<_>>()
		.join("\n\n")
}

fn md_to_bbcode(node: &Node, definitions: &Definitions) -> Option<String> {
	match node {
		Node::Root(root) => Some(handle_root(root, definitions)),
		Node::BlockQuote(quote) => Some(handle_quote(quote, definitions)),
		Node::FootnoteDefinition(_) => handle_warning("FootnoteDefinition"),
		Node::MdxJsxFlowElement(_) => handle_warning("MdxJsFlowElement"),
		Node::List(list) => Some(handle_list(list, definitions)),
		Node::MdxjsEsm(_) => handle_warning("MdxjsEsm"),
		Node::Toml(_) => handle_warning("Toml"),
		Node::Yaml(_) => handle_warning("Yaml"),
		Node::Break(_) => Some(handle_break()),
		Node::InlineCode(code) => Some(handle_inline_code(code)),
		Node::InlineMath(math) => Some(handle_inline_math(math)),
		Node::Delete(delete) => Some(handle_delete(delete, definitions)),
		Node::Emphasis(emphasis) => Some(handle_emphasis(emphasis, definitions)),
		Node::MdxTextExpression(_) => handle_warning("MdxTextExpression"),
		Node::FootnoteReference(_) => handle_warning("FootnoteReference"),
		Node::Html(_) => handle_warning("HTML"),
		Node::Image(image) => Some(handle_image(image)),
		Node::ImageReference(image) => Some(handle_image_reference(image, definitions)),
		Node::MdxJsxTextElement(_) => handle_warning("MdxJsxTextElement"),
		Node::Link(link) => Some(handle_link(link, definitions)),
		Node::LinkReference(link) => Some(handle_link_reference(link, definitions)),
		Node::Strong(strong) => Some(handle_strong(strong, definitions)),
		Node::Text(text) => Some(text.value.clone()),
		Node::Code(code) => Some(handle_code(code)),
		Node::Math(math) => Some(handle_math(math)),
		Node::MdxFlowExpression(_) => handle_warning("MdxFlowExpression"),
		Node::Heading(heading) => Some(handle_heading(heading, definitions)),
		Node::Table(_) => handle_warning("Table"),
		Node::ThematicBreak(_) => Some(handle_thematic_break()),
		Node::TableRow(_) => handle_warning("TableRow"),
		Node::TableCell(_) => handle_warning("TableCell"),
		Node::ListItem(list_item) => Some(handle_list_item(list_item, definitions)),
		Node::Definition(_) => None,
		Node::Paragraph(paragraph) => Some(handle_paragraph(paragraph, definitions)),
	}
}

fn handle_warning(node: &str) -> Option<String> {
	let msg = format!("WARNING: unsupported syntax skipped: {}", node);
	print_error(&msg, ErrColor::Yellow);
	None
}

fn handle_child_nodes(nodes: &[Node], definitions: &Definitions, separator: &str) -> String {
	nodes
		.iter()
		.filter_map(|node| md_to_bbcode(node, definitions))
		.collect::<Vec<_>>()
		.join(separator)
}

fn handle_root(root: &Root, definitions: &Definitions) -> String {
	handle_child_nodes(&root.children, definitions, "")
}

fn handle_quote(blockquote: &BlockQuote, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&blockquote.children, definitions, "\n\n");
	format!("[quote]\n{text}\n[/quote]")
}

fn handle_list(list: &List, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&list.children, definitions, "\n");
	match list.ordered {
		true => format!("[list=1]\n{text}\n[/list]"),
		false => format!("[list]\n{text}\n[/list]"),
	}
}

fn handle_break() -> String {
	"\n".into()
}

fn handle_inline_code(code: &InlineCode) -> String {
	format!("[code]{}[/code]", code.value)
}

fn handle_inline_math(math: &InlineMath) -> String {
	format!("[math]{}[/math]", math.value)
}

fn handle_delete(delete: &Delete, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&delete.children, definitions, "");
	format!("[s]{text}[/s]")
}

fn handle_emphasis(emphasis: &Emphasis, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&emphasis.children, definitions, "");
	format!("[i]{text}[/i]")
}

fn handle_image(image: &Image) -> String {
	format!("[img]{}[/img]", image.url)
}

fn handle_image_reference(image: &ImageReference, definitions: &Definitions) -> String {
	let url = definitions.get(&image.identifier).unwrap();
	format!("[img]{}[/img]", url)
}

fn handle_link(link: &Link, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&link.children, definitions, "");
	format!("[url={}]{text}[/url]", link.url)
}

fn handle_link_reference(link: &LinkReference, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&link.children, definitions, "");
	let url = definitions.get(&link.identifier).unwrap();
	format!("[url={url}]{text}[/url]")
}

fn handle_strong(strong: &Strong, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&strong.children, definitions, "");
	format!("[b]{text}[/b]")
}

fn handle_code(code: &Code) -> String {
	if let Some(lang) = &code.lang {
		format!("[codeblock={lang}]\n{}\n[/codeblock]", code.value)
	} else {
		format!("[codeblock]\n{}\n[/codeblock]", code.value)
	}
}

fn handle_math(math: &Math) -> String {
	format!("[mathblock]\n{}\n[/mathblock]", math.value)
}

fn handle_heading(heading: &Heading, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&heading.children, definitions, "");
	format!("[h{l}]\n{text}\n[/h{l}]", l = heading.depth)
}

fn handle_thematic_break() -> String {
	"[hr]".into()
}

fn handle_list_item(list_item: &ListItem, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&list_item.children, definitions, "");
	format!("[*]{text}")
}

fn handle_definition(definition: &Definition) -> (String, String) {
	(definition.identifier.clone(), definition.url.clone())
}

fn handle_paragraph(paragraph: &Paragraph, definitions: &Definitions) -> String {
	handle_child_nodes(&paragraph.children, definitions, "")
}
