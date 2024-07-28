use super::error::Result;
use super::number_format::format_number_unit_f64;

pub fn format_size_bits(bytes: f64) -> Result<String> {
	let units = [
		"b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb", "Rb", "Qb",
	];
	format_number_unit_f64(bytes * 8.0, 1000.0, &units, 2, true)
}

pub fn format_size_bytes(bytes: f64) -> Result<String> {
	let units = [
		"B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB",
	];
	format_number_unit_f64(bytes, 1000.0, &units, 2, true)
}

pub fn format_size_ibibits(bytes: f64) -> Result<String> {
	let units = [
		"b", "Kib", "Mib", "Gib", "Tib", "Pib", "Eib", "Zib", "Yib", "Rib", "Qib",
	];
	format_number_unit_f64(bytes * 8.0, 1024.0, &units, 2, true)
}

pub fn format_size_ibibytes(bytes: f64) -> Result<String> {
	let units = [
		"B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB",
	];
	format_number_unit_f64(bytes, 1024.0, &units, 2, true)
}
