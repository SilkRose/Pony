use git2::{Repository, ResetType, build};
use pony::bytes::{FormatType, format_size_bytes};
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::json::{JsonFormat, format_json};
use pony::markdown::WarningType::Quiet;
use pony::markdown::bbcode;
use pony::number_format::format_number_u128;
use pony::regex::matches;
use pony::traits::OrderedVector;
use pony::word_stats::{count_matches, word_count};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{MAIN_SEPARATOR, Path};
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
	prompts: T,
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

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq)]
struct StoryMeta {
	title: String,
	short_desc: String,
	long_desc_hash: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let dist_temp = "./dist";
	let pony_temp = "./pony-temp";
	let git_url = "https://github.com/SilkRose/Pony.git";
	let repo = setup_branch(pony_temp, git_url, "mane")?;
	setup_branch(dist_temp, git_url, "api")?;
	fs::File::create("./dist/.nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	let commits = count_commits(repo)?;
	let files = find_files_in_dir("./", true)?;
	let dirs = find_dirs_in_dir("./", true)?;
	let text = story_words(&files)?;
	let stats = commit_stats(commits, &files, &dirs, &text)?;
	let pony = pony_stats(&stats)?;
	fs::File::create("../dist/api/v1/pony.json")?
		.write_all(format_json(&pony, &JsonFormat::Tab)?.as_bytes())?;
	let characters = character_stats(&text)?;
	fs::File::create("../dist/api/v1/characters.json")?
		.write_all(format_json(&characters, &JsonFormat::Tab)?.as_bytes())?;
	update_fimfiction_stories(&files)?;
	Ok(())
}

fn count_commits(repo: Repository) -> Result<usize, Box<dyn Error>> {
	let head = repo.head()?;
	let head_commit = head.peel_to_commit()?;
	let mut revwalk = repo.revwalk()?;
	revwalk.push(head_commit.id())?;
	Ok(revwalk.count())
}

fn setup_branch(dir: &str, repo_path: &str, branch: &str) -> Result<Repository, Box<dyn Error>> {
	let repo = match Repository::open(dir) {
		Ok(repo) => {
			let mut remote = repo.find_remote("origin")?;
			remote.fetch(&[branch], None, None)?;
			let refname = format!("refs/remotes/origin/{branch}");
			let object = repo.revparse_single(&refname)?;
			repo.reset(&object, ResetType::Hard, None)?;
			drop(remote);
			drop(object);
			repo
		}
		Err(_) => {
			if Path::new(dir).exists() {
				fs::remove_dir_all(dir)?;
			}
			let mut repo = build::RepoBuilder::new();
			repo.branch(branch);
			repo.clone(repo_path, Path::new(dir))?
		}
	};
	Ok(repo)
}

fn count_blogs(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*archive[/\\]blogs[/\\].*\.md$")?);
	let blogs = files
		.iter()
		.filter(|file| matches(file, &includes, &None))
		.count();
	Ok(blogs)
}

fn count_code(files: &[String]) -> Result<usize, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*code[/\\]src[/\\].*\.rs$")?);
	let code = files
		.iter()
		.filter(|file| matches(file, &includes, &None))
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
	let includes = Some(Regex::new(&format!(r".*stories[/\\]{name}\.md$"))?);
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
	let root = Some(Regex::new(r"^\.[/\\]stories.*")?);
	let stories_dir = dirs.iter().find(|&dir| matches(dir, &root, &None));
	let mut stories = Vec::new();
	for dir in stories_dir.into_iter() {
		let dirs = find_dirs_in_dir(dir, false)?;
		for dir in dirs {
			if let Some(story_name) = dir.split(MAIN_SEPARATOR).next_back() {
				stories.push(story_name.to_string());
			}
		}
	}
	Ok(stories.sort_and_dedup_vec().len())
}

fn story_words(files: &[String]) -> Result<String, Box<dyn Error>> {
	let includes = Some(Regex::new(r"[/\\](flash-fiction|stories)[/\\].*\.md$")?);
	let excludes = Some(Regex::new(
		r"stories[/\\](ideas|names|prompts)\.md$|-meta\.md$",
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
	Ok(text)
}

fn commit_stats(
	commits: usize, files: &[String], dirs: &[String], text: &str,
) -> Result<Stats<usize>, Box<dyn Error>> {
	Ok(Stats {
		blogs: count_blogs(files)?,
		code: count_code(files)?,
		commits,
		covers: count_covers(files)?,
		flash_fiction: count_flash_fiction(files)?,
		ideas: count_specified_lines(files, "ideas", "## ")?,
		names: count_specified_lines(files, "names", "- ")?,
		prompts: count_specified_lines(files, "prompts", "- ")?,
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
		prompts: format_number_u128(stats.prompts.try_into()?)?,
		size: format_size_bytes(stats.size as f64, FormatType::Abbreviation)?,
		stories: format_number_u128(stats.stories.try_into()?)?,
		words: format_number_u128(stats.words.try_into()?)?,
	})
}

fn update_fimfiction_stories(files: &[String]) -> Result<(), Box<dyn Error>> {
	let mut story_meta_file = File::options()
		.create(true)
		.truncate(true)
		.read(true)
		.write(true)
		.open("../dist/api/v1/story-meta.json")?;
	let mut story_meta_str = String::new();
	story_meta_file.read_to_string(&mut story_meta_str)?;
	let mut story_metadata: HashMap<u32, StoryMeta> = match story_meta_str.is_empty() {
		false => serde_json::from_str(&story_meta_str)?,
		true => HashMap::default(),
	};
	let meta_regex = Some(Regex::new(
		r"^\.[/\\]stories[/\\][a-z0-9-]+[/\\][a-z0-9-]+-meta\.md$",
	)?);
	let mut meta_files = files
		.iter()
		.filter(|&file| matches(file, &meta_regex, &None))
		.collect::<Vec<_>>();
	meta_files.sort();
	let mut regex_str = String::default();
	regex_str.push_str(r"(?m)^# \[(?<title>.+)\]\(https:\/\/www\.fimfiction\.net\/story\/(?<fimfic_id>\d{1,10})(\/[a-z0-9-]*)?\)$");
	regex_str.push_str(r"(.*\n)*^## Description:\n(?<description>(.|\n)*)$");
	regex_str.push_str(r"\n## Short Description:\n(?<short_description>.*)$");
	let re = Regex::new(&regex_str)?;
	let mut update_metadata = vec![];
	for meta in meta_files {
		let file = fs::read_to_string(meta)?;
		if let Some(caps) = re.captures(&file) {
			let long_desc = bbcode::parse(caps["description"].trim(), &Quiet);
			let data = StoryMeta {
				title: caps["title"].to_string(),
				short_desc: caps["short_description"].to_string(),
				long_desc_hash: const_hex::encode(Sha256::digest(long_desc)),
			};
			let id: u32 = caps["fimfic_id"].parse()?;
			match story_metadata.entry(id) {
				Entry::Occupied(mut entry) => {
					if entry.get() != &data {
						entry.insert(data);
					}
				}
				Entry::Vacant(entry) => {
					entry.insert(data);
					update_metadata.push(id);
				}
			}
		}
	}
	update_metadata.sort();
	for updates in &update_metadata {
		// update fimfic here
	}
	// Un-comment line below when finished.
	// story_meta_file.write_all(format_json(&story_metadata, &JsonFormat::Tab)?.as_bytes())?;
	Ok(())
}
