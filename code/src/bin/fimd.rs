use applejack::regex_filter::find_files_in_dir;
use camino::Utf8Path;
use fimdoc::parser::{parse, WarningType};
use rayon::prelude::*;
use regex::Regex;
use std::fs;

fn main() {
	if Utf8Path::new("./publish").is_dir() {
		fs::remove_dir_all("./publish").unwrap()
	}
	fs::create_dir("./publish").unwrap();
	let includes = Some(vec![
		Regex::new(r"stories|flash-fiction").unwrap(),
		Regex::new(r".md$").unwrap(),
	]);
	let excludes = Some(vec![
		Regex::new(r"archive").unwrap(),
		Regex::new(r"ideas.md$|names.md$|README.md$").unwrap(),
	]);
	find_files_in_dir("../", true, &includes, &excludes)
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
