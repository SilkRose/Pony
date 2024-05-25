use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};

pub enum Json {
	Format,
	Minify,
}

pub enum Indent {
	Tab,
	Space(u8),
}

pub fn json_formatter(json_data: String, fmt_type: Json, indent: Indent) -> String {
	let indent: String = match indent {
		Indent::Tab => "\t".to_string(),
		Indent::Space(indent_number) => " ".repeat(indent_number as usize),
	};
	match fmt_type {
		Json::Format => format_json(&json_data, &indent),
		Json::Minify => minify_json(&json_data),
	}
}

fn format_json(json: &str, indent: &str) -> String {
	let value = parse_to_value(json);
	// json.len() is not ideal but its a _goodish_ default
	let mut writer = Vec::with_capacity(json.len());
	let formatter = PrettyFormatter::with_indent(indent.as_bytes());
	let mut serialiser = Serializer::with_formatter(&mut writer, formatter);
	value
		.serialize(&mut serialiser)
		.expect("Failed to serialize json data.");
	writer.push(b'\n');
	String::from_utf8(writer).expect("Failed to convert utf8 to string.")
}

fn minify_json(json: &str) -> String {
	let value = parse_to_value(json);
	serde_json::to_string(&value).expect("Failed to stringify json.")
}

#[inline]
fn parse_to_value(json: &str) -> Value {
	serde_json::from_str(json).expect("Failed to parse json.")
}
