use camino::Utf8Path;
use std::error::Error;

/// Find files function, takes in a dir, and a closure for what to include.
pub fn find_files_in_dir(dir: &str, recursive: bool) -> Result<Vec<String>, Box<dyn Error>> {
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files.extend(find_files_in_dir(&path, recursive)?);
		} else if utf8_path.is_file() {
			files.push(path);
		}
	}
	Ok(files)
}

/// Find dirs function, takes in a dir, and a closure for what to include.
pub fn find_dirs_in_dir(dir: &str, recursive: bool) -> Result<Vec<String>, Box<dyn Error>> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			dirs.push(path.clone());
			if recursive {
				dirs.extend(find_dirs_in_dir(&path, recursive)?)
			}
		}
	}
	Ok(dirs)
}
