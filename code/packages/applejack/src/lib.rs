#![deny(missing_docs)]
#![doc = include_str!("../readme.md")]

use camino::Utf8Path;
use std::io;

/// Find files function, takes in a dir, and a closure for what to include.
pub fn find_files_in_dir<F>(
	dir: &str, recursive: bool, filter: Option<F>,
) -> io::Result<Vec<String>>
where
	F: Fn(&str) -> bool + Clone,
{
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files.extend(find_files_in_dir(&path, recursive, filter.clone())?);
		} else if utf8_path.is_file() {
			if let Some(ref filter) = filter {
				if filter(&path) {
					files.push(path);
				}
			} else {
				files.push(path);
			}
		}
	}
	Ok(files)
}

/// Find dirs function, takes in a dir, and a closure for what to include.
pub fn find_dirs_in_dir<F>(dir: &str, recursive: bool, filter: Option<F>) -> io::Result<Vec<String>>
where
	F: Fn(&str) -> bool + Clone,
{
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			if let Some(ref filter) = filter {
				if filter(&path) {
					dirs.push(path);
					if recursive {
						dirs.extend(find_dirs_in_dir(dir, recursive, Some(filter))?)
					}
				}
			} else {
				dirs.push(path);
			}
		}
	}
	Ok(dirs)
}
