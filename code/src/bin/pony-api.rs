use camino::Utf8Path;
use pony::command::execute_command;
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::regex::matches;
use pony::text_stats::word_count;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{env, error::Error, fs, io::Write, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
	covers: usize,
	flash_fiction: usize,
	ideas: usize,
	names: usize,
	stories: usize,
	words: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	let dist_temp = "./dist";
	let pony_temp = "./pony-temp";
	// let url = "https://github.com/SilkRose/Pony";
	if Utf8Path::new(dist_temp).exists() {
		fs::remove_dir_all(dist_temp)?
	}
	if Utf8Path::new(pony_temp).exists() {
		fs::remove_dir_all(pony_temp)?
	}
	execute_command(&format!(
		"git clone --quiet --depth 1 --branch api https://github.com/SilkRose/Pony.git {}",
		dist_temp
	))?;
	execute_command(&format!(
		"git clone --quiet --depth 1 --branch mane https://github.com/SilkRose/Pony.git {}",
		pony_temp
	))?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	println!("{:#?}", count_covers()?);
	let stats = Stats {
		covers: count_covers()?,
		flash_fiction: count_flash_fiction()?,
		ideas: count_ideas()?,
		names: count_names()?,
		stories: count_stories()?,
		words: count_words()?,
	};
	println!("{:#?}", stats);
	Ok(())
}

fn count_covers() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(external-covers|stories).*cover.*").unwrap());
	let excludes = Some(Regex::new(r".*(archive/stories|concept|upscaled).*|\.xcf$").unwrap());
	let covers = find_files_in_dir("./", true)?;
	let mut covers = covers
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.filter_map(|cover| Path::new(cover).parent()?.to_str())
		.collect::<Vec<_>>();
	covers.sort();
	covers.dedup();
	Ok(covers.len())
}

fn count_flash_fiction() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*flash-fiction.*\.md$").unwrap());
	let flash_fiction = find_files_in_dir("./", true)?;
	let flash_fiction = flash_fiction
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.collect::<Vec<_>>();
	Ok(flash_fiction.len())
}

fn count_ideas() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(src)?[/\\]stories[/\\]ideas\.md$").unwrap());
	let ideas = find_files_in_dir("./", true)?;
	let ideas = ideas
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.collect::<Vec<_>>();
	if let Some(ideas) = ideas.first() {
		let mut text = String::new();
		let mut file = fs::File::open(ideas)?;
		file.read_to_string(&mut text)?;
		let ideas = text
			.split('\n')
			.filter(|line| line.starts_with("## "))
			.collect::<Vec<_>>()
			.len();
		Ok(ideas)
	} else {
		Ok(0)
	}
}

fn count_names() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(src)?[/\\]stories[/\\]names\.md$").unwrap());
	let names = find_files_in_dir("./", true)?;
	let names = names
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.collect::<Vec<_>>();
	if let Some(names) = names.first() {
		let mut text = String::new();
		let mut file = fs::File::open(names)?;
		file.read_to_string(&mut text)?;
		let names = text
			.split('\n')
			.filter(|line| line.starts_with("- "))
			.collect::<Vec<_>>()
			.len();
		Ok(names)
	} else {
		Ok(0)
	}
}

fn count_stories() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r"stories").unwrap());
	let excludes = Some(Regex::new(r"archive").unwrap());
	let stories_dir = find_dirs_in_dir("./", true)?
		.into_iter()
		.filter(|dir| matches(dir, &includes, &excludes))
		.collect::<Vec<_>>();
	let stories_dir = stories_dir.first();
	if let Some(stories_dir) = stories_dir {
		return Ok(find_dirs_in_dir(stories_dir, false)?.len());
	}
	Ok(0)
}

fn count_words() -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r"(flash-fiction|stories).*\.md$").unwrap());
	let excludes = Some(Regex::new(r"archive|stories[/\\](ideas|names)\.md$|meta\.md$").unwrap());
	let files = find_files_in_dir("./", true)?;
	let text = files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.map(|file| {
			let mut text = String::new();
			fs::File::open(file)?.read_to_string(&mut text)?;
			Ok::<_, Box<dyn Error>>(text)
		})
		.collect::<Result<Vec<_>, _>>()?
		.join("\n");
	Ok(word_count(text))
}
