use camino::Utf8Path;
use pony::fs::find_files_in_dir;
use pony::markdown::WarningType;
use pony::markdown::bbcode::parse;
use pony::regex::matches;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	if Utf8Path::new("./publish").is_dir() {
		fs::remove_dir_all("./publish")?
	}
	fs::create_dir("./publish")?;
	let includes = Some(Regex::new(r".*\.md$")?);
	let excludes = Some(Regex::new(r".*[/\\]code[/\\].*")?);
	find_files_in_dir("../", true)?
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.for_each(|input| {
			let md = fs::read_to_string(input).unwrap();
			let bbcode = parse(&md, &WarningType::Quiet);
			let output = input.replace("../", "./publish/").replace(".md", ".txt");
			fs::create_dir_all(Utf8Path::new(&output).parent().unwrap()).unwrap();
			fs::write(output, bbcode).unwrap();
			let html = pony::markdown::html::parse(&md, &WarningType::Quiet);
			let output = input.replace("../", "./publish/").replace(".md", ".html");
			fs::create_dir_all(Utf8Path::new(&output).parent().unwrap()).unwrap();
			fs::write(output, html).unwrap();
			println!("Converted: {input}");
		});
	fix_blog_2025_06_02a()?;
	fix_blog_2026_04_01a()?;
	Ok(())
}

fn fix_blog_2025_06_02a() -> Result<(), Box<dyn Error>> {
	let path = "./publish/archive/blogs/2025/06/02a.txt";
	let text = fs::read_to_string(path).unwrap();
	let (intro, stories) = text.split_once("[hr]").unwrap();
	let mut output = format!("{intro}[hr]\n\n");
	for line in stories.lines() {
		if line.starts_with("[h3]") {
			let link = line
				.trim_start_matches("[h3][url=")
				.split_once(']')
				.unwrap()
				.0;
			output.push_str(&format!("[center][embed]{link}[/embed][/center]\n"));
		} else if !line.is_empty() {
			output.push_str(&format!("[quote]{line}[/quote]\n\n[hr]\n\n"));
		}
	}
	fs::write(path, output).unwrap();
	Ok(())
}

fn fix_blog_2026_04_01a() -> Result<(), Box<dyn Error>> {
	let path = "./publish/archive/blogs/2026/04/01a.txt";
	let text = fs::read_to_string(path).unwrap();
	let (intro, stories) = text.split_once("[hr]").unwrap();
	let mut output = format!("{intro}[hr]\n\n");
	for line in stories.lines() {
		if line.starts_with("[h3]") {
			let link = line
				.trim_start_matches("[h3][url=")
				.split_once(']')
				.unwrap()
				.0;
			output.push_str(&format!("[center][embed]{link}[/embed][/center]\n"));
		} else if !line.is_empty() {
			output.push_str(&format!("[quote]{line}[/quote]\n\n[hr]\n\n"));
		}
	}
	fs::write(path, output).unwrap();
	Ok(())
}
