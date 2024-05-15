use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn sleep(start_time: u128, interval: u128) {
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
