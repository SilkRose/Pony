use super::error::Result;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer};
use std::{fs, io::Write, path::Path};

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

pub fn convert_type<T: Serialize, U: DeserializeOwned>(value: &T) -> Result<U> {
	let json_string = serde_json::to_string(value)?;
	let result = serde_json::from_str(&json_string)?;
	Ok(result)
}

pub fn load_json<T: DeserializeOwned>(path: &str) -> Result<T> {
	Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

pub fn load_json_option<T: DeserializeOwned>(path: &str) -> Result<Option<T>> {
	if Path::new(path).is_file() {
		let data: T = serde_json::from_str(&fs::read_to_string(path)?)?;
		Ok(Some(data))
	} else {
		Ok(None)
	}
}

pub fn save_json<T: Serialize>(path: &str, format: JsonFormat, json: &T) -> Result<()> {
	fs::File::create(path)?.write_all(format_json(&json, format)?.as_bytes())?;
	Ok(())
}
