use chrono::{DateTime, Utc};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::MAIN_SEPARATOR;
use std::sync::{Arc, Mutex};
use std::time::Duration;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Logger {
	pub console: Option<LogLevel>,
	pub file: Option<LogFile>,
}

#[derive(Debug, Clone)]
pub struct LogFile {
	pub file: Arc<Mutex<String>>,
	pub file_limit: FileLimit,
	pub dir: String,
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

	/* 	pub fn set_file(mut self, path: &str, level: LogLevel, limit: FileLimit) -> Self {
		let file = LogFile {
			path: path.to_string(),
			level,
			limit,
		};
		self.file = Some(file);
		self
	} */

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
		if let Some(log) = self.file
			&& log.level as u8 >= level as u8
		{
			let path = log.file.lock().map_err(|_| "Failed to lock data")?;
			let mut file = OpenOptions::new()
				.read(true)
				.append(true)
				.create(true)
				.open(path.as_str())?;
			writeln!(file, "{msg}").map_err(|e| format!("Failed to write to file: {e}"))?;
			let reader = BufReader::new(file);
			let mut cutoff: Option<usize> = None;
			if let FileLimit::Lines(lines) = log.file_limit {
				let count = reader.lines().count();
				if count > lines {
					cutoff = Some(count - lines)
				}
			} else if let FileLimit::Duration(duration) = log.file_limit {
				let date_cutoff = time - duration;
				for (i, line) in reader.lines().enumerate() {
					let date = DateTime::parse_from_rfc3339(&line?[0..25])?.to_utc();
					if date > date_cutoff {
						cutoff = Some(i);
						break;
					}
				}
			}
			if let Some(cutoff) = cutoff {
				// do file stuff here
				println!("{cutoff}");
			}
		}
		Ok(())
	}
}

impl Default for Logger {
	fn default() -> Self {
		let mut log = Self::new(LogLevel::Info);
		log.file = Some(LogFile::default());
		log
	}
}

impl Default for LogFile {
	fn default() -> Self {
		Self {
			file: Arc::new(Mutex::new(String::from("console.log"))),
			dir: format!(".{MAIN_SEPARATOR}logs"),
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
