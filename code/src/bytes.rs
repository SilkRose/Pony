pub fn format_size_bits(bytes: usize) -> String {
	let units = [
		"b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb", "Rb", "Qb",
	];
	format_size(bytes << 3, 1000.0, &units)
}

pub fn format_size_bytes(bytes: usize) -> String {
	let units = [
		"B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB",
	];
	format_size(bytes, 1000.0, &units)
}

pub fn format_size_ibibytes(bytes: usize) -> String {
	let units = [
		"B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB",
	];
	format_size(bytes, 1024.0, &units)
}

fn format_size(bytes: usize, divisor: f64, units: &[&str]) -> String {
	let mut current = bytes as f64;
	if current < divisor {
		return format!("{bytes:.2} {}", units[0]);
	}
	for unit in units.iter().skip(1) {
		if current <= divisor {
			return format!("{current:.2} {unit}");
		}
		current /= divisor;
	}
	format!("{bytes:.2} {}", units[units.len() - 1])
}
