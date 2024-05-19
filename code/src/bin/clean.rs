use pony::fs::find_files_in_dir;
use pony::regex::matches;
use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(stories|flash-fiction).*\.md$")?);
	let excludes = Some(Regex::new(r".*archive.*|(-meta|readme)\.md$")?);
	let single = Regex::new(r"[‘’´ʹ]")?;
	let double = Regex::new(r"[“”‟″]")?;
	find_files_in_dir("../", true)?
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
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
	Ok(())
}
