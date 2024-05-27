use super::error::Result;
use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};

pub enum JsonFormat {
	Minify,
	Tab,
	Space(u8),
}

pub fn format_json<T: Serialize>(value: &T, format: JsonFormat) -> Result<String> {
	match format {
		JsonFormat::Minify => Ok(serde_json::to_string(value)?),
		JsonFormat::Tab => Ok(format_json_indent(value, "\t")?),
		JsonFormat::Space(s) => Ok(format_json_indent(value, &" ".repeat(s.into()))?),
	}
}

fn format_json_indent<T: Serialize>(value: &T, indent: &str) -> Result<String> {
	let mut writer = Vec::new();
	let formatter = PrettyFormatter::with_indent(indent.as_bytes());
	let mut serializer = Serializer::with_formatter(&mut writer, formatter);
	value.serialize(&mut serializer)?;
	Ok(String::from_utf8(writer)?)
}
