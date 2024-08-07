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
}

impl<T: Into<ErrorInner>> From<T> for Error {
	fn from(inner: T) -> Self {
		Self {
			inner: inner.into(),
		}
	}
}
