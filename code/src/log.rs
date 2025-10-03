use crate::fs::find_files_in_dir;
use camino::Utf8PathBuf;
use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions, remove_file};
use std::io::{BufRead, BufReader, Write};
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt, fs};

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.9f";

#[derive(Debug)]
pub struct Logger {
	pub console: Option<LogLevel>,
	pub file: Option<Arc<Mutex<LogFile>>>,
}

#[derive(Debug)]
pub struct LogFile {
	pub file: Option<File>,
	pub file_lines: usize,
	pub file_line_limit: FileLimit,
	pub file_timestamp: Option<DateTime<Utc>>,
	pub dir: Utf8PathBuf,
	pub dir_limit: usize,
	pub level: LogLevel,
}

#[derive(Debug, Clone)]
pub enum FileLimit {
	Lines(usize),
	Duration(Duration),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
	Debug,
	Info,
	Warn,
	Error,
}

impl Logger {
	pub fn new(level: LogLevel) -> Self {
		Logger {
			console: Some(level),
			file: None,
		}
	}

	pub fn set_file(
		mut self, dir: &str, level: LogLevel, line_limit: FileLimit, dir_limit: usize,
	) -> Result<Self> {
		let mut timestamp: Option<DateTime<Utc>> = None;
		let mut file: Option<File> = None;
		let mut lines = 0;
		fs::create_dir_all(dir)?;
		let mut files = find_files_in_dir(dir, false)?;
		if !files.is_empty() {
			files.sort();
			let file_path = files.last().expect("Will never be none");
			let path = OpenOptions::new()
				.append(true)
				.create(true)
				.open(file_path)?;
			file = Some(path);
			let path = OpenOptions::new().read(true).open(file_path)?;
			let buffer = BufReader::new(path);
			lines = buffer.lines().count();
			if let Some(stem) = Utf8PathBuf::from(file_path).file_stem()
				&& let Ok(time) = DateTime::parse_from_str(stem, TIME_FORMAT)
			{
				timestamp = Some(time.to_utc())
			}
		}
		let log = LogFile {
			file,
			file_lines: lines,
			file_line_limit: line_limit,
			file_timestamp: timestamp,
			dir: Utf8PathBuf::from(dir),
			dir_limit,
			level,
		};
		self.file = Some(Arc::new(Mutex::new(log)));
		Ok(self)
	}

	pub fn debug(&self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Debug)
	}

	pub fn info(&self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Info)
	}

	pub fn warn(&self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Warn)
	}

	pub fn error(&self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Error)
	}

	fn log(&self, message: &str, level: LogLevel) -> Result<()> {
		let time = Utc::now();
		let msg = format!("{} - {level}: {message}", time.format(TIME_FORMAT));
		if let Some(console) = self.console
			&& console as u8 <= level as u8
		{
			println!("{msg}");
		}
		if let Some(log) = &self.file {
			let mut log = log.lock().map_err(|_| "Failed to lock data")?;
			if log.level as u8 > level as u8 {
				return Ok(());
			}
			if let FileLimit::Lines(lines) = log.file_line_limit {
				if log.file_lines >= lines {
					log.file = None;
				}
			} else if let FileLimit::Duration(duration) = log.file_line_limit
				&& let Some(timestamp) = log.file_timestamp
				&& timestamp + duration < time
			{
				log.file = None;
			}
			if log.file.is_none() {
				let path = log.dir.join(format!("{}.log", time.format(TIME_FORMAT)));
				let file = OpenOptions::new().append(true).create(true).open(path)?;
				log.file = Some(file);
				log.file_timestamp = Some(time);
				log.file_lines = 0;
				let files = find_files_in_dir(log.dir.as_str(), false)?;
				let mut logs = files
					.iter()
					.filter_map(|file| {
						if let Some(stem) = Utf8PathBuf::from(file).file_stem()
							&& let Ok(time) = DateTime::parse_from_str(stem, TIME_FORMAT)
						{
							return Some(time.to_utc());
						}
						None
					})
					.collect::<Vec<_>>();
				logs.sort();
				for file in logs.iter().take(logs.len() - log.dir_limit) {
					let path = log.dir.join(format!("{file}.log"));
					remove_file(path)?;
				}
			}
			let mut file = log.file.as_ref().expect("file will always be present");
			writeln!(file, "{msg}").map_err(|e| format!("Failed to write to file: {e}"))?;
			log.file_lines += 1;
		}
		Ok(())
	}
}

impl Default for Logger {
	fn default() -> Self {
		let mut log = Self::new(LogLevel::Info);
		log.file = Some(Arc::new(Mutex::new(LogFile::default())));
		log
	}
}

impl Default for LogFile {
	fn default() -> Self {
		Self {
			file: None,
			file_lines: 0,
			file_line_limit: FileLimit::Lines(5_000),
			file_timestamp: None,
			dir: Utf8PathBuf::from(format!(".{MAIN_SEPARATOR}logs")),
			level: LogLevel::Warn,
			dir_limit: 10,
		}
	}
}

impl fmt::Display for LogLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let text = match self {
			LogLevel::Debug => "debug",
			LogLevel::Info => "info",
			LogLevel::Warn => "warn",
			LogLevel::Error => "error",
		};
		write!(f, "{text}")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::remove_dir_all;
	use std::io::{BufRead, BufReader};
	use std::sync::LazyLock;

	type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

	#[test]
	fn test_line_limit() -> Result<()> {
		static LOG: LazyLock<Logger> = LazyLock::new(|| {
			Logger::new(LogLevel::Debug)
				.set_file("./test-line", LogLevel::Debug, FileLimit::Lines(5), 10)
				.expect("Should never fail")
		});

		for i in 1..10 {
			LOG.info(&i.to_string())?;
		}
		let logger = LOG
			.file
			.as_ref()
			.unwrap()
			.lock()
			.map_err(|_| "Failed to lock data")?;

		let path = logger.dir.join(format!(
			"{}.log",
			logger.file_timestamp.unwrap().format(TIME_FORMAT)
		));
		let file = OpenOptions::new().read(true).open(path)?;
		let reader = BufReader::new(file);
		for (i, line) in reader.lines().enumerate() {
			assert!(line?.ends_with(&(i + 6).to_string()));
		}
		remove_dir_all(logger.dir.clone())?;
		Ok(())
	}

	#[test]
	fn test_max_files() -> Result<()> {
		static LOG: LazyLock<Logger> = LazyLock::new(|| {
			Logger::new(LogLevel::Debug)
				.set_file("./test-files", LogLevel::Debug, FileLimit::Lines(5), 10)
				.expect("Should never fail")
		});
		for i in 1..=50 {
			LOG.info(&i.to_string())?;
		}
		let logger = LOG
			.file
			.as_ref()
			.unwrap()
			.lock()
			.map_err(|_| "Failed to lock data")?;

		let files = find_files_in_dir(logger.dir.as_str(), false)?;
		assert_eq!(10, files.len());
		remove_dir_all(logger.dir.clone())?;
		Ok(())
	}
}
