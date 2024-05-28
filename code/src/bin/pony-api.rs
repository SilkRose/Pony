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

enum Argument {
	CheckBinary,
	CheckJson,
	RebuildBinary,
	RebuildJson,
}

#[derive(Debug, Deserialize, Serialize)]
struct Commit {
	hash: String,
	unix_time: usize,
	message: String,
	stats: Stats,
}

#[derive(Debug, Deserialize, Serialize)]
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
	let _arg = parse_argument(&env::args().skip(1).collect::<Vec<_>>());
	let dist_temp = "./dist";
	let pony_temp = "./pony-temp";
	let repo = "https://github.com/SilkRose/Pony.git";
	if Utf8Path::new(dist_temp).exists() {
		fs::remove_dir_all(dist_temp)?
	}
	if Utf8Path::new(pony_temp).exists() {
		fs::remove_dir_all(pony_temp)?
	}
	execute_command(&format!(
		"git clone --quiet --depth 1 --branch api {repo} {dist_temp}"
	))?;
	execute_command(&format!(
		"git clone --quiet --branch mane {repo} {pony_temp}",
	))?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	let files = find_files_in_dir("./", true)?;
	let local_hash = hash_api_src(&files)?;
	let remote_hash = match Path::new("../hash.txt").is_file() {
		true => {
			let mut hash = String::new();
			fs::File::open("../hash.txt")?.read_to_string(&mut hash)?;
			Some(hash)
		}
		false => None,
	};
	if local_hash != remote_hash.unwrap_or_default() {
		// rebuild needed
		env::set_current_dir(Path::new("./code"))?;
		fs::File::create("../../hash.txt")?.write_all(local_hash.as_bytes())?;
		let status = execute_command("cargo build --release --bin pony-api")?.success();
		println!("needs_cached=true");
		exit(if status { 0 } else { 1 });
	}
	let commits = execute_command_with_return("git log mane --format=\"%H\n%ct\n%s\n\"")?;
	let binding = String::from_utf8_lossy(&commits.stdout);
	let text = binding.trim();
	let mut text = text.split("\n\n").collect::<Vec<_>>();
	text.reverse();
	let mut pony_commits: Vec<Commit> = Vec::with_capacity(text.len());
	for (index, commit) in text.iter().enumerate() {
		println!("{commit}");
		let log = commit.split('\n').collect::<Vec<_>>();
		let hash = log[0].to_string();
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
		println!("{:#?}", commit_data);
		pony_commits.push(commit_data);
	}
	pony_commits.reverse();
	fs::File::create("../dist/api/v1/pony-commits.json")?
		.write_all(format_json(&pony_commits, JsonFormat::Tab)?.as_bytes())?;
	let latest = pony_commits.first().unwrap();
	let pony = PonyStats {
		blogs: format_number_u128(latest.stats.blogs.try_into()?)?,
		code: format_number_u128(latest.stats.code.try_into()?)?,
		commits: format_number_u128(latest.stats.commits.try_into()?)?,
		covers: format_number_u128(latest.stats.covers.try_into()?)?,
		flash_fiction: format_number_u128(latest.stats.flash_fiction.try_into()?)?,
		ideas: format_number_u128(latest.stats.ideas.try_into()?)?,
		names: format_number_u128(latest.stats.names.try_into()?)?,
		size: format_size_bytes(latest.stats.size)?,
		stories: format_number_u128(latest.stats.stories.try_into()?)?,
		words: format_number_u128(latest.stats.words.try_into()?)?,
	};
	println!("{:#?}", pony);
	fs::File::create("../dist/api/v1/pony.json")?
		.write_all(format_json(&pony, JsonFormat::Tab)?.as_bytes())?;
	Ok(())
}

fn parse_argument(args: &[String]) -> Argument {
	if args.is_empty() {
		print_error("No argument provided!", ErrColor::Red);
		print_help();
		exit(1);
	}
	if args.len() > 1 {
		print_error("Too many arguments provided!", ErrColor::Red);
		print_help();
		exit(1);
	}
	match args.first().unwrap().as_str() {
		"-rb" | "--rebuild-binary" => Argument::RebuildBinary,
		"-cb" | "--check-binary" => Argument::CheckBinary,
		"-rj" | "--rebuild-json" => Argument::RebuildJson,
		"-cj" | "--check-json" => Argument::CheckJson,
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

		Builds the API json or binary for Silk Rose's Pony repository.

		Usage Examples:
		  pony-api --check-binary
		  pony-api --rebuild-json

		Options:
		  -rb, --rebuild-binary    Rebuilds the binary
		  -cb, --check-binary      Check binary for rebuild
		  -rj, --rebuild-json      Rebuild the json
		  -cj, --check-json        Check the json for updating
		  -h,  --help              Print help
		  -v,  --version           Print version\n",
		env!("CARGO_BIN_NAME"),
		env!("CARGO_PKG_VERSION")
	}
}

fn hash_api_src(files: &[String]) -> Result<String, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*[/\\]code[/\\](.*\.rs|Cargo.toml)$")?);
	let excludes = Some(Regex::new(r"archive")?);
	let mut hasher = blake3::Hasher::new();
	for path in files
		.iter()
		.filter(|file| matches(file, &includes, &excludes))
	{
		let mut bytes = Vec::new();
		fs::File::open(path)?.read_to_end(&mut bytes)?;
		hasher.update(path.as_bytes());
		hasher.update(&bytes);
	}
	let hash = hasher.finalize();
	Ok(hash.to_string())
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
	let excludes = Some(Regex::new(r".*(\.obsidian|\.git|archive).*")?);
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
