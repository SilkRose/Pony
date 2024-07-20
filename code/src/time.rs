use super::error::Result;
use crate::number_format::format_number_u128;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct TimeUnit {
	pub single: &'static str,
	pub multiple: &'static str,
	pub milliseconds_in_unit: u128,
}

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

pub fn format_milliseconds(ms: u128, max_units: Option<u8>) -> Result<String> {
	let units = vec![
		TimeUnit {
			single: "millisecond",
			multiple: "milliseconds",
			milliseconds_in_unit: 1,
		},
		TimeUnit {
			single: "second",
			multiple: "seconds",
			milliseconds_in_unit: 1_000,
		},
		TimeUnit {
			single: "minute",
			multiple: "minutes",
			milliseconds_in_unit: 60_000,
		},
		TimeUnit {
			single: "hour",
			multiple: "hours",
			milliseconds_in_unit: 3_600_000,
		},
		TimeUnit {
			single: "day",
			multiple: "days",
			milliseconds_in_unit: 86_400_000,
		},
		TimeUnit {
			single: "week",
			multiple: "weeks",
			milliseconds_in_unit: 604_800_000,
		},
		TimeUnit {
			single: "month",
			multiple: "months",
			milliseconds_in_unit: 2_629_746_000,
		},
		TimeUnit {
			single: "year",
			multiple: "years",
			milliseconds_in_unit: 31_556_952_000,
		},
		TimeUnit {
			single: "decade",
			multiple: "decades",
			milliseconds_in_unit: 315_569_520_000,
		},
		TimeUnit {
			single: "century",
			multiple: "centuries",
			milliseconds_in_unit: 3_155_695_200_000,
		},
		TimeUnit {
			single: "millennium",
			multiple: "millennia",
			milliseconds_in_unit: 31_556_952_000_000,
		},
	];
	let mut ms = ms;
	let mut times = units
		.iter()
		.rev()
		.filter_map(|unit| {
			if ms > unit.milliseconds_in_unit {
				ms %= unit.milliseconds_in_unit;
				let count = ms / unit.milliseconds_in_unit;
				let name = if count == 1 {
					unit.single
				} else {
					unit.multiple
				};
				let count = if count >= 1000 {
					format_number_u128(count).unwrap()
				} else {
					count.to_string()
				};
				Some(format!("{count} {name}"))
			} else {
				None
			}
		})
		.collect::<Vec<String>>();
	if let Some(max_units) = max_units {
		times = times.into_iter().take(max_units.into()).collect();
	}
	let time = match times.len() {
		2 => format!("{} and {}", times[0], times[1]),
		n if n > 2 => {
			if let Some(time) = times.last_mut() {
				*time = format!("and {time}");
			}
			times.join(", ")
		}
		_ => times.first().cloned().unwrap_or_default(),
	};
	Ok(time)
}
