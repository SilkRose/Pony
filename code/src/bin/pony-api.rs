use indoc::printdoc;
use pony::bytes::format_size_bytes;
use pony::command::{execute_command, execute_command_with_return};
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::json::{format_json, JsonFormat};
use pony::number_format::format_number_u128;
use pony::regex::matches;
use pony::stderr::{print_error, ErrColor};
use pony::traits::{BasicVector, OrderedVector};
use pony::word_stats::{count_matches, word_count};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{Read, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::process::exit;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Commit {
	hash: String,
	unix_time: usize,
	message: String,
	stats: Stats,
	stat_changes: StatChanges,
	chars: Characters,
	files: Vec<Files>,
	keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Stats {
	blogs: usize,
	code: usize,
	commits: usize,
	covers: usize,
	flash_fiction: usize,
	ideas: usize,
	names: usize,
	size: usize,
	stories: usize,
	words: usize,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
struct StatChanges {
	blogs: isize,
	code: isize,
	commits: isize,
	covers: isize,
	flash_fiction: isize,
	ideas: isize,
	names: isize,
	size: isize,
	stories: isize,
	words: isize,
}

#[derive(Debug, Deserialize, Serialize)]
struct PonyStats {
	blogs: String,
	code: String,
	commits: String,
	covers: String,
	flash_fiction: String,
	ideas: String,
	names: String,
	size: String,
	stories: String,
	words: String,
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

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Files {
	name: String,
	lines_added: usize,
	lines_removed: usize,
	change_type: Type,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
enum Type {
	Merge,
	Modified,
	Added,
	Deleted,
	Renamed(u8, String),
}

fn main() -> Result<(), Box<dyn Error>> {
	let mut rebuild = parse_argument(&env::args().skip(1).collect::<Vec<_>>());
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
	let commits = execute_command_with_return("git log mane --format=\"%H\n%ct\n%s\n\"")?;
	let binding = String::from_utf8_lossy(&commits.stdout);
	let text = binding.trim();
	let text = text.split("\n\n").collect::<Vec<_>>().reverse_vec();
	let json_path = "../dist/api/v1/pony-commits.json";
	let pony_commits_json: Option<Vec<Commit>> = if Path::new(json_path).is_file() && !rebuild {
		let commits: Vec<Commit> = serde_json::from_str(&fs::read_to_string(json_path)?)?;
		Some(commits.reverse_vec())
	} else {
		rebuild = true;
		None
	};
	let pony_commits = pony_commit_stats(text, rebuild, pony_commits_json)?;
	fs::File::create("../dist/api/v1/pony-commits.json")?
		.write_all(format_json(&pony_commits, JsonFormat::Tab)?.as_bytes())?;
	let pony = pony_stats(&pony_commits.first().unwrap().stats)?;
	fs::File::create("../dist/api/v1/pony.json")?
		.write_all(format_json(&pony, JsonFormat::Tab)?.as_bytes())?;
	Ok(())
}

fn parse_argument(args: &[String]) -> bool {
	if args.is_empty() {
		return false;
	}
	if args.len() > 1 {
		print_error("Too many arguments provided!", ErrColor::Red);
		print_help();
		exit(1);
	}
	match args.first().unwrap().as_str() {
		"-r" | "--rebuild" => true,
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

fn pony_commit_stats(
	text: Vec<&str>, rebuild: bool, pony_commits_json: Option<Vec<Commit>>,
) -> Result<Vec<Commit>, Box<dyn Error>> {
	let mut pony_commits: Vec<Commit> = Vec::with_capacity(text.len());
	for (index, commit) in text.iter().enumerate() {
		let log = commit.split('\n').collect::<Vec<_>>();
		let hash = log[0].to_string();
		if let Some(ref commits) = pony_commits_json {
			if !rebuild && commits.get(index).is_some() && hash == commits.get(index).unwrap().hash
			{
				pony_commits.push(commits.get(index).unwrap().clone());
				continue;
			}
		}
		let unix_time = log[1].parse::<usize>()?;
		let message = log[2].to_string();
		execute_command(&format!("git checkout --quiet {hash}"))?;
		let files = find_files_in_dir("./", true)?;
		let dirs = find_dirs_in_dir("./", true)?;
		let text = story_words(&files)?;
		let stats = commit_stats(index, &files, &dirs, &text)?;
		let stat_changes = stat_changes(&pony_commits, &stats)?;
		let chars = character_stats(&text)?;
		let file_changes = file_changes(&hash)?;
		let keywords = keywords(&stat_changes, &file_changes)?;
		let commit_data = Commit {
			hash,
			unix_time,
			message,
			stats,
			stat_changes,
			chars,
			files: file_changes,
			keywords,
		};
		pony_commits.push(commit_data);
	}
	Ok(pony_commits.reverse_vec())
}

fn commit_stats(
	index: usize, files: &[String], dirs: &[String], text: &str,
) -> Result<Stats, Box<dyn Error>> {
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

fn stat_changes(commits: &[Commit], current: &Stats) -> Result<StatChanges, Box<dyn Error>> {
	let changes = match commits.len() > 1 {
		true => {
			let previous = &commits.last().unwrap().stats;
			StatChanges {
				blogs: current.blogs as isize - previous.blogs as isize,
				code: current.code as isize - previous.code as isize,
				commits: current.commits as isize - previous.commits as isize,
				covers: current.covers as isize - previous.covers as isize,
				flash_fiction: current.flash_fiction as isize - previous.flash_fiction as isize,
				ideas: current.ideas as isize - previous.ideas as isize,
				names: current.names as isize - previous.names as isize,
				size: current.size as isize - previous.size as isize,
				stories: current.stories as isize - previous.stories as isize,
				words: current.words as isize - previous.words as isize,
			}
		}
		false => StatChanges {
			blogs: current.blogs.try_into()?,
			code: current.code.try_into()?,
			commits: current.commits.try_into()?,
			covers: current.covers.try_into()?,
			flash_fiction: current.flash_fiction.try_into()?,
			ideas: current.ideas.try_into()?,
			names: current.names.try_into()?,
			size: current.size.try_into()?,
			stories: current.stories.try_into()?,
			words: current.words.try_into()?,
		},
	};
	Ok(changes)
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

fn file_changes(hash: &str) -> Result<Vec<Files>, Box<dyn Error>> {
	let num_stats = execute_command_with_return(&format!(
		"git show --pretty=\"\" --numstat {hash} | sed 's/\\s\\+/ /'",
	))?;
	let binding = String::from_utf8_lossy(&num_stats.stdout);
	let text = binding.trim();
	let num_stats = text.split('\n').collect::<Vec<_>>();
	let name_status = execute_command_with_return(&format!(
		"git show --pretty=\"\" --name-status {hash} | sed 's/\\s\\+/ /'",
	))?;
	let binding = String::from_utf8_lossy(&name_status.stdout);
	let text = binding.trim();
	let name_status = text.split('\n').collect::<Vec<_>>();
	if num_stats.len() != name_status.len() {
		panic!("git show command output different lenght data!")
	}
	let stats = num_stats
		.iter()
		.zip(name_status)
		.map(|(num_stats, name_status)| {
			let num_stats = num_stats.split_whitespace().collect::<Vec<_>>();
			let name_status = name_status.split_whitespace().collect::<Vec<_>>();
			let lines_added = num_stats.first().unwrap().parse::<usize>().unwrap_or(0);
			let lines_removed = num_stats.get(1).unwrap().parse::<usize>().unwrap_or(0);
			let change_type_char = name_status.first().unwrap_or(&"C").chars().next().unwrap();
			let name = name_status
				.get(1)
				.unwrap_or(num_stats.last().unwrap())
				.to_string();
			let change_type = match change_type_char {
				'C' => Type::Merge,
				'A' => Type::Added,
				'D' => Type::Deleted,
				'M' => Type::Modified,
				'R' => {
					let new_name = name_status.last().unwrap().to_string();
					let percentage = name_status.first().unwrap()[1..].parse::<u8>().unwrap();
					Type::Renamed(percentage, new_name)
				}
				_ => panic!("Encountered wrong letter in git output!"),
			};
			Files {
				name,
				lines_added,
				lines_removed,
				change_type,
			}
		})
		.collect::<Vec<Files>>();
	Ok(stats)
}

fn pony_stats(stats: &Stats) -> Result<PonyStats, Box<dyn Error>> {
	Ok(PonyStats {
		blogs: format_number_u128(stats.blogs.try_into()?)?,
		code: format_number_u128(stats.code.try_into()?)?,
		commits: format_number_u128(stats.commits.try_into()?)?,
		covers: format_number_u128(stats.covers.try_into()?)?,
		flash_fiction: format_number_u128(stats.flash_fiction.try_into()?)?,
		ideas: format_number_u128(stats.ideas.try_into()?)?,
		names: format_number_u128(stats.names.try_into()?)?,
		size: format_size_bytes(stats.size)?,
		stories: format_number_u128(stats.stories.try_into()?)?,
		words: format_number_u128(stats.words.try_into()?)?,
	})
}

fn keywords(stat_changes: &StatChanges, files: &[Files]) -> Result<Vec<String>, Box<dyn Error>> {
	let mut keywords = vec![];
	// Keyword ideas:
	// story, cover, writing, proofreading, management
	// code, coding, pony, ideas, names, meta, repo
	// workflow, misc, blog, obsidian, templates, merge
	// archive, deletion, refactoring, editing, readme
	// license, addition, modification, flash fiction
	// renaming, github, promotions, banner, props
	// image, text, configeration, featured images
	// markdown, rust, toml, yaml, json, python
	// typescript, gnuplot, shell, png, jpg, xcf
	// ase, gif, lock file, gitignore, gitattributes
	//
	// handling ideas: 1 file min, 1 dir min
	// special files, speciial dirs
	// percent min, percent max
	if stat_changes.stories > 1 {
		keywords.push("Story addition".to_string());
	} else if stat_changes.stories < 0 {
		keywords.push("Story deletion".to_string());
	}
	let stories_inc = Some(Regex::new(r"stories")?);
	if reduce(files, &stories_inc, &None) >= 0.50 {
		keywords.push("Story".to_string());
	}
	Ok(keywords)
}

fn reduce(files: &[Files], includes: &Option<Regex>, excludes: &Option<Regex>) -> f64 {
	files
		.iter()
		.filter(|file| match &file.change_type {
			Type::Merge => false,
			Type::Renamed(_, name) => {
				matches(&file.name, includes, excludes) || matches(name, includes, excludes)
			}
			_ => matches(&file.name, includes, excludes),
		})
		.count() as f64
		/ files.len() as f64
}
