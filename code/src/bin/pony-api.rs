use indoc::printdoc;
use pony::bytes::{format_size_bytes, FormatType};
use pony::command::{execute_command, execute_command_with_return};
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::json::{format_json, JsonFormat};
use pony::number_format::format_number_u128;
use pony::regex::matches;
use pony::stderr::{print_error, ErrColor};
use pony::traits::OrderedVector;
use pony::word_stats::{count_matches, word_count};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{Read, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::process::exit;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Stats<T> {
	blogs: T,
	code: T,
	commits: T,
	covers: T,
	flash_fiction: T,
	ideas: T,
	names: T,
	size: T,
	stories: T,
	words: T,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Characters {
	applejack: usize,
	fluttershy: usize,
	pinkie_pie: usize,
	rainbow_dash: usize,
	rarity: usize,
	twilight_sparkle: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
	parse_argument(&env::args().skip(1).collect::<Vec<_>>());
	let dist_temp = "./dist";
	let pony_temp = "./pony-temp";
	let repo = "https://github.com/SilkRose/Pony.git";
	let dist_cmd = format!("git clone --quiet --branch api {repo} {dist_temp}");
	let pony_cmd = format!("git clone --quiet --branch mane {repo} {pony_temp}");
	setup_branch(dist_temp, &dist_cmd, "api")?;
	setup_branch(pony_temp, &pony_cmd, "mane")?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	let commits = execute_command_with_return("git log mane --format=\"%H%n%ct%n%at%n%s%n\"")?;
	let binding = String::from_utf8_lossy(&commits.stdout);
	let text = binding.trim();
	let index = text.split("\n\n").collect::<Vec<_>>().len() - 1;
	let files = find_files_in_dir("./", true)?;
	let dirs = find_dirs_in_dir("./", true)?;
	let text = story_words(&files)?;
	let stats = commit_stats(index, &files, &dirs, &text)?;
	let pony = pony_stats(&stats)?;
	fs::File::create("../dist/api/v1/pony.json")?
		.write_all(format_json(&pony, JsonFormat::Tab)?.as_bytes())?;
	let characters = character_stats(&text)?;
	fs::File::create("../dist/api/v1/characters.json")?
		.write_all(format_json(&characters, JsonFormat::Tab)?.as_bytes())?;
	Ok(())
}

fn parse_argument(args: &[String]) {
	if args.is_empty() {
		return;
	}
	if args.len() > 1 {
		print_error("Too many arguments provided!", ErrColor::Red);
		print_help();
		exit(1);
	}
	match args.first().unwrap().as_str() {
		"-h" | "--help" => {
			print_help();
			exit(0);
		}
		"-v" | "--version" => {
			println!("{} {}", env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
			exit(0);
		}
		_ => {
			print_error("Incorrect argument provided!", ErrColor::Red);
			print_help();
			exit(1);
		}
	}
}

fn print_help() {
	printdoc! {"
		{} {}

		Builds the API json for Silk Rose's Pony repository.

		Usage Examples:
		  pony-api
		  pony-api --rebuild

		Options:
		  -r, --rebuild      Rebuild the pony-commits.json file
		  -h,  --help        Print help
		  -v,  --version     Print version\n",
		env!("CARGO_BIN_NAME"),
		env!("CARGO_PKG_VERSION")
	}
}

fn setup_branch(dir: &str, cmd: &str, branch: &str) -> Result<(), Box<dyn Error>> {
	if Path::new(dir).exists() {
		let status = execute_command(&format!(
			"cd {dir} && git fetch --quiet && git reset --quiet --hard origin/{branch} && git pull --quiet"
		))?;
		if !status.success() {
			fs::remove_dir_all(dir)?
		} else {
			return Ok(());
		};
	};
	let status = execute_command(cmd)?;
	if !status.success() {
		return Err(format!("Failed to execute command: {cmd}").into());
	};
	Ok(())
}

fn count_blogs(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(archive)?[/\\]blogs?[/\\].*\.md$")?);
	let blogs = files
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.count();
	Ok(blogs)
}

fn count_code(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*\.(sh|py|ts|gp|rs)$")?);
	let excludes = Some(Regex::new(r".*(\.obsidian|\.git).*")?);
	let code = files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.map(|file| {
			let mut text = String::new();
			fs::File::open(file)?.read_to_string(&mut text)?;
			Ok::<_, Box<dyn Error>>(text)
		})
		.collect::<Result<Vec<_>, _>>()?;
	let code = code
		.iter()
		.flat_map(|file| file.split('\n').map(|line| line.trim()))
		.collect::<Vec<_>>()
		.sort_and_dedup_vec();
	Ok(code.len())
}

fn count_covers(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(external-covers|stories).*cover.*")?);
	let excludes = Some(Regex::new(r".*(concept|upscaled).*|\.xcf$|\.ase$")?);
	let covers = files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
		.filter_map(|cover| Path::new(cover).parent()?.to_str())
		.collect::<Vec<_>>()
		.sort_and_dedup_vec();
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

fn count_size(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let excludes = Some(Regex::new(
		r".*(\.obsidian|\.git|code/(target|publish|dist|pony-temp)).*",
	)?);
	let bytes = files
		.iter()
		.filter(|file| matches(file, &None, &excludes))
		.map(|file| Ok::<_, Box<dyn Error>>(fs::File::open(file)?.metadata()?.len() as usize))
		.collect::<Result<Vec<_>, _>>()?
		.into_iter()
		.sum();
	Ok(bytes)
}

fn count_stories(dirs: &[String]) -> Result<usize, Box<dyn Error>> {
	let archive = Some(Regex::new(r"^\.[/\\]archive[/\\]stories.*")?);
	let root = Some(Regex::new(r"^\.[/\\](src[/\\])?stories.*")?);
	let stories_archive = dirs.iter().find(|&dir| matches(dir, &archive, &None));
	let stories_root = dirs.iter().find(|&dir| matches(dir, &root, &None));
	let mut stories = Vec::new();
	for dir in [stories_archive, stories_root].into_iter().flatten() {
		let dirs = find_dirs_in_dir(dir, false)?;
		for dir in dirs {
			if let Some(story_name) = dir.split(MAIN_SEPARATOR).last() {
				stories.push(story_name.to_string());
			}
		}
	}
	Ok(stories.sort_and_dedup_vec().len())
}

fn story_words(files: &[String]) -> Result<String, Box<dyn Error>> {
	let includes = Some(Regex::new(r"[/\\](flash-fiction|stories)[/\\].*\.md$")?);
	let excludes = Some(Regex::new(r"stories[/\\](ideas|names)\.md$|meta\.md$")?);
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
	Ok(text)
}

fn commit_stats(
	index: usize, files: &[String], dirs: &[String], text: &str,
) -> Result<Stats<usize>, Box<dyn Error>> {
	Ok(Stats {
		blogs: count_blogs(files)?,
		code: count_code(files)?,
		commits: index + 1,
		covers: count_covers(files)?,
		flash_fiction: count_flash_fiction(files)?,
		ideas: count_specified_lines(files, "ideas", "## ")?,
		names: count_specified_lines(files, "names", "- ")?,
		size: count_size(files)?,
		stories: count_stories(dirs)?,
		words: word_count(text)?,
	})
}

fn character_stats(text: &str) -> Result<Characters, Box<dyn Error>> {
	Ok(Characters {
		applejack: count_matches(text, Regex::new(r"Applejack|AJ")?),
		fluttershy: count_matches(text, Regex::new(r"Fluttershy|Flutters")?),
		pinkie_pie: count_matches(text, Regex::new(r"Pinkie( Pie)?")?),
		rainbow_dash: count_matches(text, Regex::new(r"Rainbow Dash|Rainbow|Dash(ie)?")?),
		rarity: count_matches(text, Regex::new(r"Rarity")?),
		twilight_sparkle: count_matches(text, Regex::new(r"Twilight Sparkle|Twi(light)?")?),
	})
}

fn pony_stats(stats: &Stats<usize>) -> Result<Stats<String>, Box<dyn Error>> {
	Ok(Stats {
		blogs: format_number_u128(stats.blogs.try_into()?)?,
		code: format_number_u128(stats.code.try_into()?)?,
		commits: format_number_u128(stats.commits.try_into()?)?,
		covers: format_number_u128(stats.covers.try_into()?)?,
		flash_fiction: format_number_u128(stats.flash_fiction.try_into()?)?,
		ideas: format_number_u128(stats.ideas.try_into()?)?,
		names: format_number_u128(stats.names.try_into()?)?,
		size: format_size_bytes(stats.size as f64, FormatType::Abbreviation)?,
		stories: format_number_u128(stats.stories.try_into()?)?,
		words: format_number_u128(stats.words.try_into()?)?,
	})
}
