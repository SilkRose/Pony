use std::error::Error;
use std::num::NonZeroUsize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// The initial duration in milliseconds to wait and the multiplier for each subsequent run.
pub type Backoff = (u128, f64);

pub fn retry<T, F>(
	mut attempts: Option<usize>, mut backoff: Option<Backoff>, function: F,
) -> Result<T, Box<dyn Error>>
where
	F: Fn() -> Result<T, Box<dyn Error>>,
{
	let mut attempt;
	loop {
		let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
		attempt = function();
		if attempt.is_ok() {
			return attempt;
		}
		if attempts.is_some() {
			attempts = Some(attempts.unwrap() - 1);
			if attempts.unwrap() == 0 {
				return attempt;
			}
		}
		if let Some((initial_delay, multiplier)) = backoff {
			sleep(start_time, initial_delay);
			backoff = Some(((initial_delay as f64 * multiplier) as u128, multiplier));
		}
	}
}

pub fn retry_max_tries<T, F>(attempts: NonZeroUsize, function: F) -> Result<T, Box<dyn Error>>
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

fn sleep(start_time: u128, interval: u128) {
	let current_time = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();
	let elapsed_time = current_time - start_time;
	println!("{elapsed_time}");
	if elapsed_time > interval {
		return;
	};
	std::thread::sleep(Duration::from_millis((interval - elapsed_time) as u64));
}
