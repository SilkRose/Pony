use super::error::Result;
use super::number_format::format_number_f64;

pub fn format_size_bits(bytes: usize) -> Result<String> {
	let units = [
		"b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb", "Rb", "Qb",
	];
	format_size(bytes << 3, 1000.0, &units)
}

pub fn format_size_bytes(bytes: usize) -> Result<String> {
	let units = [
		"B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB",
	];
	format_size(bytes, 1000.0, &units)
}

pub fn format_size_ibibits(bytes: usize) -> Result<String> {
	let units = [
		"b", "Kib", "Mib", "Gib", "Tib", "Pib", "Eib", "Zib", "Yib", "Rib", "Qib",
	];
	format_size(bytes << 3, 1024.0, &units)
}

pub fn format_size_ibibytes(bytes: usize) -> Result<String> {
	let units = [
		"B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB",
	];
	format_size(bytes, 1024.0, &units)
}

fn format_size(bytes: usize, divisor: f64, units: &[&str]) -> Result<String> {
	let mut current = bytes as f64;
	for unit in units.iter() {
		if current <= divisor {
			return Ok(format!("{current:.2} {unit}"));
		}
		current /= divisor;
	}
	Ok(format!(
		"{} {}",
		format_number_f64(current, 2)?,
		units.last().unwrap()
	))
}
