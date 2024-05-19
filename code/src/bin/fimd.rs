use camino::Utf8Path;
use pony::fs::find_files_in_dir;
use pony::md_to_bbcode::{parse, WarningType};
use pony::regex::matches;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	if Utf8Path::new("./publish").is_dir() {
		fs::remove_dir_all("./publish")?
	}
	fs::create_dir("./publish")?;
	let includes = Some(Regex::new(r".*(stories|flash-fiction).*\.md$")?);
	let excludes = Some(Regex::new(r".*archive.*|.*(ideas|names|readme)\.md$")?);
	find_files_in_dir("../", true)?
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.for_each(|input| {
			let md = fs::read_to_string(input).unwrap();
			let bbcode = parse(md, &WarningType::Quiet);
			let output = input.replace("../", "./publish/").replace(".md", ".txt");
			fs::create_dir_all(Utf8Path::new(&output).parent().unwrap()).unwrap();
			fs::write(output, bbcode).unwrap();
			println!("Converted: {input}");
		});
	Ok(())
}
