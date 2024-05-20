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
}

impl<T: Into<ErrorInner>> From<T> for Error {
	fn from(inner: T) -> Self {
		Self {
			inner: inner.into(),
		}
	}
}
