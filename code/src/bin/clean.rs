use pony::fs::find_files_in_dir;
use pony::json::{format_json, JsonFormat};
use pony::regex::matches;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let files = find_files_in_dir("../", true)?;
	let includes = Some(Regex::new(r".*(stories|flash-fiction).*\.md$")?);
	let excludes = Some(Regex::new(r".*archive.*|(-meta|readme)\.md$")?);
	let single = Regex::new(r"[‘’´ʹ]")?;
	let double = Regex::new(r"[“”‟″]")?;
	for file in files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
	{
		println!("Formatting: {file}");
		let mut data = fs::read_to_string(file)?;
		data = single.replace_all(&data, "'").to_string();
		data = double.replace_all(&data, "\"").to_string();
		data = data
			.replace("...", "…")
			.replace(",*", "*,")
			.replace(",_", "_,")
			.replace("---", "—")
			.replace("--", "–");

		fs::write(file, data)?;
	}
	let includes = Some(Regex::new(r".*\.json$")?);
	let excludes = Some(Regex::new(
		r".*(\.obsidian|\.vscode|code[/\\](dist|pony-temp|target)).*",
	)?);
	for file in files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
	{
		println!("Formatting: {file}");
		let data: Value = serde_json::from_str(&fs::read_to_string(file)?)?;
		fs::write(file, format_json(&data, JsonFormat::Tab)?.as_bytes())?;
	}
	Ok(())
}
