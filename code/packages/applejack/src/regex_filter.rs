use camino::Utf8Path;
use regex::Regex;
use std::io;

/// Find files function, takes in a dir, and Vectors of Regex, for what the returned files must include and exclude.
pub fn find_files_in_dir(
	dir: &str, recursive: bool, includes: &Option<Regex>, excludes: &Option<Regex>,
) -> io::Result<Vec<String>> {
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files.extend(find_files_in_dir(&path, recursive, includes, excludes)?);
		} else if utf8_path.is_file() {
			if requirements_met(&path, includes, excludes) {
				files.push(path);
			} else {
				continue;
			}
		}
	}
	Ok(files)
}

/// Find dirs function, takes in a dir, and Vectors of Regex, for what the returned dirs must include and exclude.
pub fn find_dirs_in_dir(
	dir: &str, recursive: bool, includes: &Option<Regex>, excludes: &Option<Regex>,
) -> io::Result<Vec<String>> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			if requirements_met(&path, includes, excludes) {
				dirs.push(path);
				if recursive {
					dirs.extend(find_dirs_in_dir(dir, recursive, includes, excludes)?)
				}
			} else {
				continue;
			}
		}
	}
	Ok(dirs)
}

fn requirements_met(string: &str, includes: &Option<Regex>, excludes: &Option<Regex>) -> bool {
	if let Some(excludes) = excludes {
		if excludes.is_match(string) {
			return false;
		}
	}
	if let Some(includes) = includes {
		return includes.is_match(string);
	}
	true
}
