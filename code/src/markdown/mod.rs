use std::collections::HashMap;

pub mod bbcode;
pub mod html;
pub mod plaintext;

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

type Definitions = HashMap<String, String>;

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
