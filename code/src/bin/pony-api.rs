use camino::Utf8Path;
use pony::command::{execute_command, execute_command_with_return};
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::regex::matches;
use pony::word_stats::word_count;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{env, error::Error, fs, io::Write, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
	blogs: usize,
	code: usize,
	commits: usize,
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
		"git clone --quiet --branch mane https://github.com/SilkRose/Pony.git {}",
		pony_temp
	))?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	let files = find_files_in_dir("./", true)?;
	let dirs = find_dirs_in_dir("./", true)?;
	let stats = Stats {
		blogs: count_blogs(&files)?,
		code: count_code(&files)?,
		commits: count_commits()?,
		covers: count_covers(&files)?,
		flash_fiction: count_flash_fiction(&files)?,
		ideas: count_specified_lines(&files, "ideas", "## ")?,
		names: count_specified_lines(&files, "names", "- ")?,
		stories: count_stories(&dirs)?,
		words: count_words(&files)?,
	};
	println!("{:#?}", stats);
	Ok(())
}

fn count_blogs(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(archive)?[/\\]blogs[/\\].*\.md$")?);
	let blogs = files
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.filter_map(|cover| Path::new(cover).parent()?.to_str())
		.collect::<Vec<_>>();
	Ok(blogs.len())
}

fn count_code(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*[/\\]code[/\\].*\.(sh|py|ts|gp|rs)$")?);
	let excludes = Some(Regex::new(r".*(\.obsidian|\.git|archive).*")?);
	let code = files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.map(|file| {
			let mut text = String::new();
			fs::File::open(file)?.read_to_string(&mut text)?;
			Ok::<_, Box<dyn Error>>(text)
		})
		.collect::<Result<Vec<_>, _>>()?;
	let mut code = code
		.iter()
		.flat_map(|file| file.split('\n').map(|line| line.trim()))
		.collect::<Vec<_>>();
	code.sort();
	code.dedup();
	Ok(code.len())
}

fn count_commits() -> Result<usize, Box<dyn Error>> {
	let commits = execute_command_with_return(r#"git log mane --format="format:%H""#)?;
	Ok(String::from_utf8_lossy(&commits.stdout).split('\n').count())
}

fn count_covers(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(external-covers|stories).*cover.*")?);
	let excludes = Some(Regex::new(
		r".*(archive/stories|concept|upscaled).*|\.xcf$",
	)?);
	let mut covers = files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.filter_map(|cover| Path::new(cover).parent()?.to_str())
		.collect::<Vec<_>>();
	covers.sort();
	covers.dedup();
	Ok(covers.len())
}

fn count_flash_fiction(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*flash-fiction.*\.md$")?);
	let flash_fiction = files
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.collect::<Vec<_>>();
	Ok(flash_fiction.len())
}

fn count_specified_lines(
	files: &[String], name: &str, starts_with: &str,
) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(&format!(
		r".*(src)?[/\\]stories[/\\]{name}\.md$"
	))?);
	let names = files.iter().find(|file| matches(file, &includes, &None));
	if let Some(names) = names {
		let mut text = String::new();
		let mut file = fs::File::open(names)?;
		file.read_to_string(&mut text)?;
		let names = text
			.split('\n')
			.filter(|line| line.starts_with(starts_with))
			.collect::<Vec<_>>()
			.len();
		Ok(names)
	} else {
		Ok(0)
	}
}

fn count_stories(dirs: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r"stories")?);
	let excludes = Some(Regex::new(r"archive")?);
	let stories_dir = dirs.iter().find(|dir| matches(dir, &includes, &excludes));
	if let Some(stories_dir) = stories_dir {
		return Ok(find_dirs_in_dir(stories_dir, false)?.len());
	}
	Ok(0)
}

fn count_words(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r"(flash-fiction|stories).*\.md$")?);
	let excludes = Some(Regex::new(
		r"archive|stories[/\\](ideas|names)\.md$|meta\.md$",
	)?);
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
	Ok(word_count(text)?)
}
