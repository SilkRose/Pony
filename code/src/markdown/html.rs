use super::{handle_warning, Definitions, WarningType};
use markdown::mdast::{
	Blockquote, Definition, Delete, Emphasis, Heading, Image, ImageReference, InlineCode, Link,
	LinkReference, List, ListItem, Node, Paragraph, Root, Strong,
};
use markdown::{to_mdast, ParseOptions};

/// Parse function for turning markdown into FIMFiction BBCode.
pub fn parse(md: &str, warn: &WarningType) -> String {
	let node = to_mdast(md, &ParseOptions::gfm()).unwrap();
	handle_node(&node, warn)
}

fn handle_node(node: &Node, warn: &WarningType) -> String {
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
		.filter_map(|n| md_to_html(n, warn, &definitions))
		.collect::<Vec<_>>()
		.join("\n\n")
}

fn md_to_html(node: &Node, warn: &WarningType, definitions: &Definitions) -> Option<String> {
	match node {
		Node::Root(root) => Some(handle_root(root, warn, definitions)),
		Node::Blockquote(quote) => Some(handle_quote(quote, warn, definitions)),
		Node::FootnoteDefinition(_) => handle_warning("FootnoteDefinition", warn),
		Node::MdxJsxFlowElement(_) => handle_warning("MdxJsFlowElement", warn),
		Node::List(list) => Some(handle_list(list, warn, definitions)),
		Node::MdxjsEsm(_) => handle_warning("MdxjsEsm", warn),
		Node::Toml(_) => handle_warning("Toml", warn),
		Node::Yaml(_) => handle_warning("Yaml", warn),
		Node::Break(_) => Some(handle_break()),
		Node::InlineCode(code) => Some(handle_inline_code(code)),
		Node::InlineMath(_) => handle_warning("InlineMath", warn),
		Node::Delete(delete) => Some(handle_delete(delete, warn, definitions)),
		Node::Emphasis(emphasis) => Some(handle_emphasis(emphasis, warn, definitions)),
		Node::MdxTextExpression(_) => handle_warning("MdxTextExpression", warn),
		Node::FootnoteReference(_) => handle_warning("FootnoteReference", warn),
		Node::Html(_) => handle_warning("HTML", warn),
		Node::Image(image) => Some(handle_image(image)),
		Node::ImageReference(image) => Some(handle_image_reference(image, definitions)),
		Node::MdxJsxTextElement(_) => handle_warning("MdxJsxTextElement", warn),
		Node::Link(link) => Some(handle_link(link, warn, definitions)),
		Node::LinkReference(link) => Some(handle_link_reference(link, warn, definitions)),
		Node::Strong(strong) => Some(handle_strong(strong, warn, definitions)),
		Node::Text(text) => Some(text.value.clone()),
		Node::Code(_) => handle_warning("Code", warn),
		Node::Math(_) => handle_warning("Math", warn),
		Node::MdxFlowExpression(_) => handle_warning("MdxFlowExpression", warn),
		Node::Heading(heading) => Some(handle_heading(heading, warn, definitions)),
		Node::Table(_) => handle_warning("Table", warn),
		Node::ThematicBreak(_) => Some(handle_thematic_break()),
		Node::TableRow(_) => handle_warning("TableRow", warn),
		Node::TableCell(_) => handle_warning("TableCell", warn),
		Node::ListItem(list_item) => Some(handle_list_item(list_item, warn, definitions)),
		Node::Definition(_) => None,
		Node::Paragraph(paragraph) => Some(handle_paragraph(paragraph, warn, definitions)),
	}
}

fn handle_child_nodes(
	nodes: &[Node], warn: &WarningType, definitions: &Definitions, separator: &str,
) -> String {
	nodes
		.iter()
		.filter_map(|node| md_to_html(node, warn, definitions))
		.collect::<Vec<_>>()
		.join(separator)
}

fn handle_root(root: &Root, warn: &WarningType, definitions: &Definitions) -> String {
	handle_child_nodes(&root.children, warn, definitions, "")
}

fn handle_quote(blockquote: &Blockquote, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&blockquote.children, warn, definitions, "\n\n");
	format!("<blockquote>\n{text}\n</blockquote>")
}

fn handle_list(list: &List, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&list.children, warn, definitions, "\n");
	match list.ordered {
		true => format!("<ol>\n{text}\n</ol>"),
		false => format!("<ul>\n{text}\n</ul>"),
	}
}

fn handle_break() -> String {
	"<br />".into()
}

fn handle_inline_code(code: &InlineCode) -> String {
	format!("<code>{}</code>", code.value)
}

fn handle_delete(delete: &Delete, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&delete.children, warn, definitions, "");
	format!("<s>{text}</s>")
}

fn handle_emphasis(emphasis: &Emphasis, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&emphasis.children, warn, definitions, "");
	format!("<i>{text}</i>")
}

fn handle_image(image: &Image) -> String {
	format!("<img src=\"{}\" alt=\"{}\" ></img>", image.url, image.alt)
}

fn handle_image_reference(image: &ImageReference, definitions: &Definitions) -> String {
	let url = definitions.get(&image.identifier).unwrap();
	format!("<img src=\"{url}\" alt=\"{}\" ></img>", image.alt)
}

fn handle_link(link: &Link, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&link.children, warn, definitions, "");
	format!("<a href=\"{}\">{text}</a>", link.url)
}

fn handle_link_reference(
	link: &LinkReference, warn: &WarningType, definitions: &Definitions,
) -> String {
	let text = handle_child_nodes(&link.children, warn, definitions, "");
	let url = definitions.get(&link.identifier).unwrap();
	format!("<a href=\"{url}\">{text}</a>")
}

fn handle_strong(strong: &Strong, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&strong.children, warn, definitions, "");
	format!("<b>{text}</b>")
}

fn handle_heading(heading: &Heading, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&heading.children, warn, definitions, "");
	format!("<h{l}>{text}</h{l}>", l = heading.depth)
}

fn handle_thematic_break() -> String {
	"<hr />".into()
}

fn handle_list_item(list_item: &ListItem, warn: &WarningType, definitions: &Definitions) -> String {
	let text = handle_child_nodes(&list_item.children, warn, definitions, "");
	format!("<li>{text}</li>")
}

fn handle_definition(definition: &Definition) -> (String, String) {
	(definition.identifier.clone(), definition.url.clone())
}

fn handle_paragraph(
	paragraph: &Paragraph, warn: &WarningType, definitions: &Definitions,
) -> String {
	handle_child_nodes(&paragraph.children, warn, definitions, "")
}
