use std::error::Error;
use std::num::NonZeroUsize;

pub fn retry<T, F>(attempts: NonZeroUsize, function: F) -> Result<T, Box<dyn Error>>
where
	F: Fn() -> Result<T, Box<dyn Error>>,
{
	let mut attempt = function();
	for _ in 1..attempts.get() {
		if attempt.is_ok() {
			return attempt;
		}
		attempt = function();
	}
	attempt
}

pub fn retry_forever<T, F>(function: F) -> Result<T, Box<dyn Error>>
where
	F: Fn() -> Result<T, Box<dyn Error>>,
{
	let mut attempt = function();
	loop {
		if attempt.is_ok() {
			return attempt;
		}
		attempt = function();
	}
}
