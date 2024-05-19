use camino::Utf8Path;
use std::{fs, io};

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

/// Find file function, takes in a dir, and a closure for the file to find.
pub fn find_file_in_dir<F>(dir: &str, recursive: bool, filter: F) -> io::Result<Option<String>>
where
	F: Fn(&str) -> bool + Clone,
{
	let paths = Utf8Path::read_dir_utf8(dir.into())?;
	for path in paths {
		let path = path?.path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			return find_file_in_dir(&path, recursive, filter.clone());
		} else if utf8_path.is_file() && filter(&path) {
			return Ok(Some(path));
		}
	}
	Ok(None)
}

/// Find dirs function, takes in a dir, and a closure for what to include.
pub fn find_dirs_in_dir(dir: &str, recursive: bool) -> io::Result<Vec<String>> {
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

/// Copy files function, takes a source and destination folder.
/// recursive option allows it to go deeper than the root source directory.
/// closure lets you filter which files get copied.
pub fn copy_files_from_dir<T, F>(
	source_dir: &str, destination_dir: &str, recursive: bool, filter: Option<F>,
) -> io::Result<()>
where
	F: Fn(&str) -> bool + Clone,
{
	let source = Utf8Path::new(source_dir);
	let dest = Utf8Path::new(destination_dir);
	let files = find_files_in_dir(source_dir, recursive, filter)?;
	for file in files {
		let destination = dest.join(Utf8Path::new(&file).strip_prefix(source).unwrap());
		fs::copy(file, destination)?;
	}
	Ok(())
}

/// Move files function, takes a source and destination folder.
/// recursive option allows it to go deeper than the root source directory.
/// closure lets you filter which files get copied.
pub fn move_files_from_dir<T, F>(
	source_dir: &str, destination_dir: &str, recursive: bool, filter: Option<F>,
) -> io::Result<()>
where
	F: Fn(&str) -> bool + Clone,
{
	let source = Utf8Path::new(source_dir);
	let dest = Utf8Path::new(destination_dir);
	let files = find_files_in_dir(source_dir, recursive, filter)?;
	for file in files {
		let destination = dest.join(Utf8Path::new(&file).strip_prefix(source).unwrap());
		fs::rename(file, destination)?;
	}
	Ok(())
}
