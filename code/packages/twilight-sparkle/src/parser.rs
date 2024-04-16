use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};

/// Parse Markdown into text, removing all Markdown syntax.
pub fn parse(md: String) -> String {
	let node = to_mdast(&md, &ParseOptions::gfm()).unwrap();
	handle_node(&node)
}

fn handle_node(node: &Node) -> String {
	node.children()
		.unwrap()
		.iter()
		.filter_map(md_to_text)
		.collect::<Vec<_>>()
		.join("\n\n")
}

fn handle_child_nodes(nodes: &[Node], separator: &str) -> String {
	nodes
		.iter()
		.filter_map(md_to_text)
		.collect::<Vec<_>>()
		.join(separator)
}

fn md_to_text(node: &Node) -> Option<String> {
	match node {
		Node::Root(root) => Some(handle_child_nodes(&root.children, "")),
		Node::BlockQuote(quote) => Some(handle_child_nodes(&quote.children, "\n")),
		Node::FootnoteDefinition(_) => None,
		Node::MdxJsxFlowElement(_) => None,
		Node::List(list) => Some(handle_child_nodes(&list.children, "\n")),
		Node::MdxjsEsm(_) => None,
		Node::Toml(_) => None,
		Node::Yaml(_) => None,
		Node::Break(_) => Some("\n".into()),
		Node::InlineCode(code) => Some(code.value.to_string()),
		Node::InlineMath(math) => Some(math.value.to_string()),
		Node::Delete(delete) => Some(handle_child_nodes(&delete.children, "")),
		Node::Emphasis(emphasis) => Some(handle_child_nodes(&emphasis.children, "")),
		Node::MdxTextExpression(_) => None,
		Node::FootnoteReference(_) => None,
		Node::Html(_) => None,
		Node::Image(image) => Some(image.alt.to_string()),
		Node::ImageReference(image) => Some(image.alt.to_string()),
		Node::MdxJsxTextElement(_) => None,
		Node::Link(link) => Some(handle_child_nodes(&link.children, "")),
		Node::LinkReference(link) => Some(handle_child_nodes(&link.children, "")),
		Node::Strong(strong) => Some(handle_child_nodes(&strong.children, "")),
		Node::Text(text) => Some(text.value.to_string()),
		Node::Code(code) => Some(code.value.to_string()),
		Node::Math(math) => Some(math.value.to_string()),
		Node::MdxFlowExpression(_) => None,
		Node::Heading(heading) => Some(handle_child_nodes(&heading.children, "")),
		Node::Table(_) => None,
		Node::ThematicBreak(_) => None,
		Node::TableRow(_) => None,
		Node::TableCell(_) => None,
		Node::ListItem(list_item) => Some(handle_child_nodes(&list_item.children, "")),
		Node::Definition(_) => None,
		Node::Paragraph(paragraph) => Some(handle_child_nodes(&paragraph.children, "")),
	}
}
