use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::MAIN_SEPARATOR;
use std::time::Duration;
use wiwi::clock_timer::chrono::Utc;

type Result<T, E = Box<dyn ::std::error::Error>> = ::std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Logger {
	pub console: Option<LogLevel>,
	pub file: Option<LogFile>,
}

#[derive(Debug, Clone)]
pub struct LogFile {
	pub path: String,
	pub level: LogLevel,
	pub limit: FileLimit,
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

	pub fn set_file(mut self, path: &str, level: LogLevel, limit: FileLimit) -> Self {
		let file = LogFile {
			path: path.to_string(),
			level,
			limit,
		};
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
		if let Some(console) = self.console
			&& console as u8 >= level as u8
		{
			let msg = format!("{time} - {level}: {message}");
			println!("{msg}");
		}
		if let Some(log_file) = self.file
			&& log_file.level as u8 >= level as u8
		{
			let msg = format!("{time} - {level}: {message}");
			let mut file = OpenOptions::new().append(true).open(log_file.path)?;
			writeln!(file, "{msg}").map_err(|e| format!("Failed to write to file: {e}"))?;
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
			path: format!(".{MAIN_SEPARATOR}console.log"),
			level: LogLevel::Warn,
			limit: FileLimit::default(),
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
