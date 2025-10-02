use crate::fs::find_files_in_dir;
use camino::Utf8PathBuf;
use chrono::{DateTime, Utc};
use std::fmt;
use std::fs::{File, OpenOptions, remove_file};
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};
use std::time::Duration;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub struct Logger {
	pub console: Option<LogLevel>,
	pub file: Option<Arc<Mutex<LogFile>>>,
}

#[derive(Debug)]
pub struct LogFile {
	pub file: Option<File>,
	pub file_limit: FileLimit,
	pub file_lines: usize,
	pub file_timestamp: Option<DateTime<Utc>>,
	pub dir: Utf8PathBuf,
	pub dir_limit: usize,
	pub level: LogLevel,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
	Debug,
	Info,
	Warn,
	Error,
}

#[derive(Debug, Clone)]
pub enum FileLimit {
	Lines(usize),
	Duration(Duration),
}

impl Logger {
	pub fn new(level: LogLevel) -> Self {
		Logger {
			console: Some(level),
			file: None,
		}
	}

	pub fn set_file(mut self, dir: &str, level: LogLevel, limit: FileLimit) -> Self {
		let file = LogFile {
			dir: Utf8PathBuf::from(dir),
			file_limit: limit,
			level,
			..Default::default()
		};
		let file = Arc::new(Mutex::new(file));
		self.file = Some(file);
		self
	}

	pub fn debug(self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Debug)
	}

	pub fn info(self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Info)
	}

	pub fn warn(self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Warn)
	}

	pub fn error(self, message: &str) -> Result<()> {
		self.log(message, LogLevel::Error)
	}

	fn log(self, message: &str, level: LogLevel) -> Result<()> {
		let time = Utc::now();
		let msg = format!("{} - {level}: {message}", time.to_rfc3339());
		if let Some(console) = self.console
			&& console as u8 >= level as u8
		{
			println!("{msg}");
		}
		if let Some(log) = self.file {
			let mut log = log.lock().map_err(|_| "Failed to lock data")?;
			if level as u8 > log.level as u8 {
				return Ok(());
			}
			if let FileLimit::Lines(lines) = log.file_limit {
				if lines > log.file_lines {
					log.file = None;
				}
			} else if let FileLimit::Duration(duration) = log.file_limit
				&& let Some(timestamp) = log.file_timestamp
				&& timestamp + duration < time
			{
				log.file = None;
			}
			if log.file.is_none() {
				let path = log.dir.join(format!("{time}.log"));
				let file = OpenOptions::new().append(true).create(true).open(path)?;
				log.file = Some(file);
				log.file_timestamp = Some(time);
				log.file_lines = 0;
				let files = find_files_in_dir(log.dir.as_str(), false)?;
				let mut logs = files
					.iter()
					.filter_map(|file| {
						if let Some(stem) = Utf8PathBuf::from(file).file_stem()
							&& let Ok(time) = DateTime::parse_from_rfc3339(stem)
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
			file_timestamp: None,
			dir: Utf8PathBuf::from(format!(".{MAIN_SEPARATOR}logs")),
			level: LogLevel::Warn,
			file_limit: FileLimit::default(),
			dir_limit: 10,
		}
	}
}

impl Default for FileLimit {
	fn default() -> Self {
		Self::Lines(50_000)
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
