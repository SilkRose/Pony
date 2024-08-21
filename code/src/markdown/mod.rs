pub mod bbcode;
pub mod html;
pub mod plaintext;

use bbcode::md_to_bbcode;
use html::md_to_html;
use markdown::{
	mdast::{Definition, Node},
	to_mdast, ParseOptions,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
/// Warning type enum for what to do when encountering unsupported markdown syntax.
pub enum WarningType {
	/// Warns the user in yellow on unsupported markdown syntax.
	Warn,
	/// Errors in red before terminating on unsupported markdown syntax.
	Fail,
	/// Ignores and skips over unsupported markdown syntax.
	Quiet,
}

#[derive(Clone, Debug)]
pub enum FormatType {
	BBCode,
	HTML,
	PlainText,
}

type Definitions = HashMap<String, String>;

pub fn parse(md: &str, warn: &WarningType, output: &FormatType) -> String {
	let node = to_mdast(md, &ParseOptions::gfm()).unwrap();
	match output {
		FormatType::BBCode => handle_node_bbcode(&node, warn),
		FormatType::HTML => handle_node_html(&node, warn),
		FormatType::PlainText => plaintext::parse(&node),
	}
}

macro_rules! handle_markdown {
	($fn1:ident, $fn2:ident, $fn3:ident, $str:literal) => {
		fn $fn1(node: &Node, warn: &WarningType) -> String {
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
				.filter_map(|node| $fn2(node, warn, &definitions))
				.collect::<Vec<_>>()
				.join($str)
		}

		fn $fn3(
			nodes: &[Node], warn: &WarningType, definitions: &Definitions, separator: &str,
		) -> String {
			nodes
				.iter()
				.filter_map(|node| $fn2(node, warn, definitions))
				.collect::<Vec<_>>()
				.join(separator)
		}
	};
}

handle_markdown!(
	handle_node_bbcode,
	md_to_bbcode,
	handle_child_nodes_bbcode,
	"\n\n"
);
handle_markdown!(
	handle_node_html,
	md_to_html,
	handle_child_nodes_html,
	"\n\n"
);

fn handle_warning(node: &str, error: &WarningType) -> Option<String> {
	match error {
		WarningType::Warn => {
			println!("WARNING: unsupported syntax skipped: {}", node);
			None
		}
		WarningType::Fail => {
			panic!("ERROR: unsupported syntax found: {}", node);
		}
		WarningType::Quiet => None,
	}
}

fn handle_definition(definition: &Definition) -> (String, String) {
	(definition.identifier.clone(), definition.url.clone())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn heading_1() {
		let md = "# Pinkie Pie";
		let bbcode = parse(md, &WarningType::Quiet, &FormatType::BBCode);
		assert_eq!("[h1]Pinkie Pie[/h1]", bbcode);
	}

	#[test]
	fn thematic_break() {
		let md = "***";
		let bbcode = parse(md, &WarningType::Quiet, &FormatType::BBCode);
		assert_eq!("[hr]", bbcode);
	}
}
