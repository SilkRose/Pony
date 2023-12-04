use pony::fs::find_files_in_dir;
use regex::Regex;

fn main() {
	let includes = Some(vec![
		Regex::new(r"stories|flash-fiction").unwrap(),
		Regex::new(r".md$").unwrap(),
	]);
	let excludes = Some(vec![
		Regex::new(r"archive").unwrap(),
		Regex::new(r"README.md$").unwrap(),
	]);
	let files = find_files_in_dir("../", true, &includes, &excludes);
	for file in files.iter() {
		println!("{file}")
	}
}
