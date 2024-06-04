use camino::Utf8Path;
use indoc::printdoc;
use pony::bytes::format_size_bytes;
use pony::command::{execute_command, execute_command_with_return};
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::json::{format_json, JsonFormat};
use pony::number_format::format_number_u128;
use pony::regex::matches;
use pony::stderr::{print_error, ErrColor};
use pony::word_stats::word_count;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone, Serialize)]
struct Commit {
	hash: String,
	unix_time: usize,
	message: String,
	stats: Stats,
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

fn main() -> Result<(), Box<dyn Error>> {
	let mut rebuild = parse_argument(&env::args().skip(1).collect::<Vec<_>>());
	let dist_temp = "./dist";
	let pony_temp = "./pony-temp";
	let repo = "https://github.com/SilkRose/Pony.git";
	let dist_cmd = format!("git clone --quiet --depth 1 --branch api {repo} {dist_temp}");
	let pony_cmd = format!("git clone --quiet --branch mane {repo} {pony_temp}");
	setup_branch(dist_temp, &dist_cmd, "api")?;
	setup_branch(pony_temp, &pony_cmd, "mane")?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	let commits = execute_command_with_return("git log mane --format=\"%H\n%ct\n%s\n\"")?;
	let binding = String::from_utf8_lossy(&commits.stdout);
	let text = binding.trim();
	let mut text = text.split("\n\n").collect::<Vec<_>>();
	text.reverse();
	let json_path = "../dist/api/v1/pony-commits.json";
	let pony_commits_json: Option<Vec<Commit>> = if Path::new(json_path).is_file() && !rebuild {
		let mut commits: Vec<Commit> = serde_json::from_str(&fs::read_to_string(json_path)?)?;
		commits.reverse();
		Some(commits)
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
	if Utf8Path::new(dir).exists() {
		let status = execute_command(&format!(
			"cd {dir} && git pull --force --quiet origin {branch}"
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

fn count_size(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let excludes = Some(Regex::new(
		r".*(\.obsidian|\.git|archive|code/(target|publish|dist|pony-temp)).*",
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
		let stats = Stats {
			blogs: count_blogs(&files)?,
			code: count_code(&files)?,
			commits: index + 1,
			covers: count_covers(&files)?,
			flash_fiction: count_flash_fiction(&files)?,
			ideas: count_specified_lines(&files, "ideas", "## ")?,
			names: count_specified_lines(&files, "names", "- ")?,
			size: count_size(&files)?,
			stories: count_stories(&dirs)?,
			words: count_words(&files)?,
		};
		let commit_data = Commit {
			hash,
			unix_time,
			message,
			stats,
		};
		pony_commits.push(commit_data);
	}
	pony_commits.reverse();
	Ok(pony_commits)
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
