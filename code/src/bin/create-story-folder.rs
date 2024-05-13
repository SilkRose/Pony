use pony::stdin::{ask, ask_longform};
use std::{error::Error, fs};

const TEMPLATE: &str = include_str!("../../../archive/markdown-templates/story-one-shot.md");

fn main() -> Result<(), Box<dyn Error>> {
	let title = ask(
		"Story name:",
		Some((
			|text: &str| !text.is_empty() && text.len() < 60,
			"Must not be empty and less than 60 characters.",
		)),
	)?;
	let synopsis = ask_longform("Synopsis:", "./tempfile.md")?;
	create_story_folder(&title, &synopsis);
	Ok(())
}

fn create_story_folder(title: &str, synopsis: &str) {
	let filename = title.to_ascii_lowercase().replace(' ', "-");
	let md = TEMPLATE
		.replace("# Title", format!("# {title}").as_str())
		.replace(
			"## Synopsis:\n\n",
			format!("## Synopsis:\n{synopsis}\n").as_str(),
		)
		.replace(
			"## Story:\n\n",
			format!("## Story:\n[{title}](./{filename}.md)\n").as_str(),
		);
	let dir = format!("../stories/{filename}");
	fs::create_dir(dir.clone()).unwrap();
	let story = format!("{dir}/{filename}.md");
	let meta = format!("{dir}/{filename}-meta.md");
	fs::write(story, format!("# {title}\n\n")).unwrap();
	fs::write(meta, md).unwrap();
}
