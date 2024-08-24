use camino::Utf8Path;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

/// Find files function, takes in a dir, and a closure for what to include.
pub fn find_files_in_dir(dir: &str, recursive: bool) -> Result<Vec<String>> {
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
pub fn find_dirs_in_dir(dir: &str, recursive: bool) -> Result<Vec<String>> {
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn files() {
		let files = find_files_in_dir("./test-dir", false).unwrap();
		assert_eq!(vec!["./test-dir/f1"], files);
	}
	#[test]
	fn files_recursive() {
		let mut files = find_files_in_dir("./test-dir", true).unwrap();
		files.sort();
		assert_eq!(
			vec!["./test-dir/d1/d2/f3", "./test-dir/d1/f2", "./test-dir/f1"],
			files
		);
	}
	#[test]
	#[should_panic]
	fn files_missing_dir() {
		let _ = find_files_in_dir("./dir-test", false).unwrap();
	}
	#[test]
	fn dirs() {
		let dirs = find_dirs_in_dir("./test-dir", false).unwrap();
		assert_eq!(vec!["./test-dir/d1"], dirs);
	}
	#[test]
	fn dirs_recursive() {
		let mut dirs = find_dirs_in_dir("./test-dir", true).unwrap();
		dirs.sort();
		assert_eq!(vec!["./test-dir/d1", "./test-dir/d1/d2"], dirs);
	}
	#[test]
	#[should_panic]
	fn dirs_missing_dir() {
		let _ = find_dirs_in_dir("./dir-test", false).unwrap();
	}
}
