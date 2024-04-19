#![deny(missing_docs)]
#![doc = include_str!("../readme.md")]

use camino::Utf8Path;
use regex::Regex;
use std::io;

/// Find files function, takes in a dir, and Vectors of Regex, for what the returned files must include and exclude.
pub fn find_files_in_dir(
	dir: &str, recursive: bool, includes: &Option<Vec<Regex>>, excludes: &Option<Vec<Regex>>,
) -> io::Result<Vec<String>> {
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files.extend(find_files_in_dir(&path, recursive, includes, excludes)?);
		} else if utf8_path.is_file() {
			if let Some(excludes) = excludes {
				if excludes.iter().any(|forbidden| forbidden.is_match(&path)) {
					continue;
				}
			}
			if let Some(includes) = includes {
				if includes.iter().all(|required| required.is_match(&path)) {
					files.push(path.to_string());
					continue;
				}
			} else {
				files.push(path);
			}
		}
	}
	Ok(files)
}

/// Find dirs function, takes in a dir, and Vectors of Regex, for what the returned dirs must include and exclude.
pub fn find_dirs_in_dir(
	dir: &str, recursive: bool, includes: &Option<Vec<Regex>>, excludes: &Option<Vec<Regex>>,
) -> io::Result<Vec<String>> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			if let Some(excludes) = excludes {
				if excludes.iter().any(|forbidden| forbidden.is_match(&path)) {
					continue;
				}
			}
			if let Some(includes) = includes {
				if includes.iter().all(|required| required.is_match(&path)) {
					dirs.push(path.to_string());
					continue;
				}
			} else {
				dirs.push(path);
				if recursive {
					dirs.extend(find_dirs_in_dir(dir, recursive, includes, excludes)?)
				}
			}
		}
	}
	Ok(dirs)
}
