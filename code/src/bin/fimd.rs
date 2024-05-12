use camino::Utf8Path;
use fancy_regex::Regex;
use pony::fs::find_files_in_dir;
use pony::md_to_bbcode::{parse, WarningType};
use pony::regex::matches;
use rayon::prelude::*;
use std::fs;

fn main() {
	if Utf8Path::new("./publish").is_dir() {
		fs::remove_dir_all("./publish").unwrap()
	}
	fs::create_dir("./publish").unwrap();
	let includes = Some(Regex::new(r".*(stories|flash-fiction).*\.md$").unwrap());
	let excludes = Some(Regex::new(r".*archive.*|.*(ideas|names|readme)\.md$").unwrap());
	find_files_in_dir(
		"../",
		true,
		Some(|path: &_| matches(path, &includes, &excludes)),
	)
	.unwrap()
	.par_iter()
	.for_each(|input| {
		let md = fs::read_to_string(input).unwrap();
		let bbcode = parse(md, &WarningType::Quiet);
		let output = input.replace("../", "./publish/").replace(".md", ".txt");
		fs::create_dir_all(Utf8Path::new(&output).parent().unwrap()).unwrap();
		fs::write(output, bbcode).unwrap();
		println!("Converted: {input}");
	});
}
