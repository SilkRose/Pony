pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
	inner: ErrorInner,
}

#[derive(Debug, thiserror::Error)]
enum ErrorInner {
	#[error(transparent)]
	Regex(#[from] regex::Error),
	#[error(transparent)]
	IO(#[from] std::io::Error),
	#[error(transparent)]
	Json(#[from] serde_json::Error),
	#[error(transparent)]
	String(#[from] std::string::FromUtf8Error),
	#[error(transparent)]
	Str(#[from] std::str::Utf8Error),
	#[error(transparent)]
	Time(#[from] std::time::SystemTimeError),
	#[error(transparent)]
	Image(#[from] image::ImageError),
	#[error(transparent)]
	Int(#[from] std::num::ParseIntError),
	#[error("Error: {0}")]
	FromString(String),
}

impl<T: Into<ErrorInner>> From<T> for Error {
	fn from(inner: T) -> Self {
		Self {
			inner: inner.into(),
		}
	}
}

impl Error {
	pub fn new(message: &str) -> Self {
		Self {
			inner: ErrorInner::FromString(message.to_string()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use image::ImageError;
	use regex::Regex;
	use serde_json;
	use std::io;
	use std::thread::sleep;
	use std::time::{Duration, SystemTime};

	#[test]
	fn test_error_new() {
		let error = Error::new("Test error message");
		if let ErrorInner::FromString(ref message) = error.inner {
			assert_eq!(message, "Test error message");
		}
	}

	#[test]
	fn test_error_from_regex_error() {
		#[allow(clippy::invalid_regex)]
		let regex_error = Regex::new("[").err().unwrap();
		let error: Error = regex_error.into();
		if let ErrorInner::Regex(_) = error.inner {
		} else {
			panic!("Expected ErrorInner::Regex");
		}
	}

	#[test]
	fn test_error_from_io_error() {
		let io_error = io::Error::new(io::ErrorKind::Other, "io error");
		let error: Error = io_error.into();
		if let ErrorInner::IO(_) = error.inner {
		} else {
			panic!("Expected ErrorInner::IO");
		}
	}

	#[test]
	fn test_error_from_json_error() {
		let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
			.err()
			.unwrap();
		let error: Error = json_error.into();
		if let ErrorInner::Json(_) = error.inner {
		} else {
			panic!("Expected ErrorInner::Json");
		}
	}

	#[test]
	fn test_error_from_utf8_error() {
		let invalid_bytes = [0x80, 0x80]; // Invalid UTF-8 sequence
		#[allow(invalid_from_utf8)]
		let utf8_error = std::str::from_utf8(&invalid_bytes).err().unwrap();
		let error: Error = utf8_error.into();
		assert!(matches!(error.inner, ErrorInner::Str(_)));
	}

	#[test]
	fn test_error_from_from_utf8_error() {
		let invalid_bytes = vec![0x80, 0x80]; // Invalid UTF-8 sequence
		let utf8_error = String::from_utf8(invalid_bytes).err().unwrap();
		let error: Error = utf8_error.into();
		assert!(matches!(error.inner, ErrorInner::String(_)));
	}

	#[test]
	fn test_error_from_system_time_error() {
		let sys_time = SystemTime::now();
		sleep(Duration::from_millis(10));
		let new_sys_time = SystemTime::now();
		match sys_time.duration_since(new_sys_time) {
			Ok(_) => panic!("Expected SystemTimeError"),
			Err(e) => {
				let error: Error = e.into();
				assert!(matches!(error.inner, ErrorInner::Time(_)));
			}
		}
	}

	#[test]
	fn test_error_from_image_error() {
		let image_error =
			ImageError::Unsupported(image::error::UnsupportedError::from_format_and_kind(
				image::error::ImageFormatHint::Unknown,
				image::error::UnsupportedErrorKind::Format(image::error::ImageFormatHint::Unknown),
			));
		let error: Error = image_error.into();
		if let ErrorInner::Image(_) = error.inner {
		} else {
			panic!("Expected ErrorInner::Image");
		}
	}

	#[test]
	fn test_error_from_parse_int_error() {
		let parse_int_error = "abc".parse::<u32>().err().unwrap();
		let error: Error = parse_int_error.into();
		if let ErrorInner::Int(_) = error.inner {
		} else {
			panic!("Expected ErrorInner::Int");
		}
	}

	#[test]
	fn test_error_from_string_error() {
		let message = "custom error message".to_string();
		let error: Error = ErrorInner::FromString(message.clone()).into();
		if let ErrorInner::FromString(ref msg) = error.inner {
			assert_eq!(msg, &message);
		} else {
			panic!("Expected ErrorInner::FromString");
		}
	}

	#[test]
	fn test_error_from_error_inner() {
		let error_inner = ErrorInner::FromString("test".to_string());
		let error: Error = error_inner.into();
		if let ErrorInner::FromString(ref msg) = error.inner {
			assert_eq!(msg, "test");
		} else {
			panic!("Expected ErrorInner::FromString");
		}
	}
}
