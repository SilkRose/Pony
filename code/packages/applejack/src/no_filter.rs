use camino::Utf8Path;
use std::io;

/// Find files function.
pub fn find_files_in_dir(dir: &str, recursive: bool) -> io::Result<Vec<String>> {
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

/// Find dirs function.
pub fn find_dirs_in_dir(dir: &str, recursive: bool) -> io::Result<Vec<String>> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			dirs.push(path);
			if recursive {
				dirs.extend(find_dirs_in_dir(dir, recursive)?)
			}
		}
	}
	Ok(dirs)
}
