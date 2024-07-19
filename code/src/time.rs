use super::error::Result;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn sleep(start_time: u128, interval: u128) -> Result<()> {
	let current_time = unix_time()?;
	let elapsed_time = current_time - start_time;
	if elapsed_time > interval {
		return Ok(());
	};
	std::thread::sleep(Duration::from_millis((interval - elapsed_time) as u64));
	Ok(())
}

pub fn unix_time() -> Result<u128> {
	Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis())
}
