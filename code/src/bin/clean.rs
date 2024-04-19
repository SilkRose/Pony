use applejack::regex_filter::find_files_in_dir;
use regex::Regex;
use std::fs;

fn main() {
	let includes = Some(Regex::new(r".*(stories|flash-fiction).*\.md$").unwrap());
	let excludes = Some(Regex::new(r".*archive.*|(-meta|readme)\.md$").unwrap());
	let single = Regex::new(r"[‘’´ʹ]").unwrap();
	let double = Regex::new(r"[“”‟″]").unwrap();
	find_files_in_dir("../", true, &includes, &excludes)
		.unwrap()
		.iter()
		.for_each(|file| {
			let mut data = fs::read_to_string(file).unwrap();
			data = single.replace_all(&data, "'").to_string();
			data = double.replace_all(&data, "\"").to_string();
			data = data
				.replace("...", "…")
				.replace(",*", "*,")
				.replace(",_", "_,")
				.replace("---", "—")
				.replace("--", "–");

			fs::write(file, data).unwrap();
		});
}
