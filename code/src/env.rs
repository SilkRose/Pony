use std::env::{self, current_dir};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

pub fn dotenv() -> Result<()> {
	let path = current_dir()?.join(".env");
	let file = OpenOptions::new().read(true).open(path)?;
	let reader = BufReader::new(file);
	for line in reader.lines() {
		let line = line?;
		let line = line.trim();
		if line.is_empty() || line.starts_with('#') {
			continue;
		}
		if let Some((key, value)) = line.split_once('=') {
			if env::var(key).is_ok() {
				continue;
			}
			unsafe { env::set_var(key.trim(), value.trim()) };
		}
	}
	Ok(())
}
