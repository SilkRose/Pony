use std::error::Error;

pub fn retry<T, F>(attempts: usize, function: F) -> Result<T, Box<dyn Error>>
where
	F: Fn() -> Result<T, Box<dyn Error>>,
{
	let mut attempt = function();
	for _ in 1..attempts {
		if attempt.is_ok() {
			return attempt;
		}
		attempt = function();
	}
	attempt
}
