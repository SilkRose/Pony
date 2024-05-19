use camino::Utf8Path;
use pony::command::execute_command;
use pony::fs::{find_dirs_in_dir, find_files_in_dir};
use pony::regex::matches;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs, io::Write, path::Path};

#[derive(Debug, Deserialize, Serialize)]
struct Stats {
	covers: i32,
	flash_fiction: i32,
	ideas: i32,
	names: i32,
	stories: i32,
	words: i32,
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
	fs::File::create("./dist/nojekyll")?;
	fs::File::create("./dist/CNAME")?.write_all(b"pony.silkrose.dev")?;
	env::set_current_dir(Path::new(pony_temp))?;
	println!("{:#?}", count_covers()?);
	// let stats = Stats {
	// 	covers: count_covers()?,
	// 	flash_fiction: todo!(),
	// 	ideas: todo!(),
	// 	names: todo!(),
	// 	stories: todo!(),
	// 	words: todo!(),
	// };
	// println!("{:#?}", stats);
	Ok(())
}

fn count_covers() -> Result<i32, Box<dyn Error>> {
	let includes = Some(Regex::new(r".*(external-covers|stories).*cover.*").unwrap());
	let excludes = Some(Regex::new(r".*(archive/stories|concept|upscaled).*|\.xcf$").unwrap());
	let covers = find_files_in_dir(
		"./",
		true,
		Some(|path: &str| matches(path, &includes, &excludes)),
	)?;
	let mut covers = covers
		.iter()
		.filter_map(|cover| Path::new(cover).parent()?.to_str())
		.collect::<Vec<_>>();
	covers.sort();
	covers.dedup();
	Ok(covers.len().try_into()?)
}
