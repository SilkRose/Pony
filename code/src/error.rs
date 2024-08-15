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
	use serde_json;
	use std::io;

	#[test]
	fn test_error_new() {
		let error = Error::new("Test error message");
		if let ErrorInner::FromString(ref message) = error.inner {
			assert_eq!(message, "Test error message");
		} else {
			panic!("Expected ErrorInner::FromString");
		}
	}

	#[test]
	fn test_error_from_io_error() {
		let io_error = io::Error::new(io::ErrorKind::Other, "io error");
		let error: Error = io_error.into();
		assert!(matches!(error.inner, ErrorInner::IO(_)));
	}

	#[test]
	fn test_error_from_json_error() {
		let json_error = serde_json::from_str::<serde_json::Value>("invalid json")
			.err()
			.unwrap();
		let error: Error = json_error.into();
		assert!(matches!(error.inner, ErrorInner::Json(_)));
	}

	#[test]
	fn test_error_from_parse_int_error() {
		let parse_int_error = "abc".parse::<u32>().err().unwrap();
		let error: Error = parse_int_error.into();
		assert!(matches!(error.inner, ErrorInner::Int(_)));
	}
}
