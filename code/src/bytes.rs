use super::error::Result;
use super::number_format::format_number_unit_f64;

pub enum FormatType {
	Abbreviation,
	FullName,
}

pub fn format_size_bits(bytes: f64, format: FormatType) -> Result<String> {
	let abbreviations = [
		"b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb", "Rb", "Qb",
	];
	let full_names = [
		"bits",
		"kilobits",
		"megabits",
		"gigabits",
		"terabits",
		"petabits",
		"exabits",
		"zettabits",
		"yottabits",
		"ronnabits",
		"quettabits",
	];

	let units = match format {
		FormatType::Abbreviation => abbreviations,
		FormatType::FullName => full_names,
	};

	format_number_unit_f64(bytes * 8.0, 1000.0, &units, 2, true)
}

pub fn format_size_bytes(bytes: f64, format: FormatType) -> Result<String> {
	let abbreviations = [
		"B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB", "RB", "QB",
	];
	let full_names = [
		"bytes",
		"kilobytes",
		"megabytes",
		"gigabytes",
		"terabytes",
		"petabytes",
		"exabytes",
		"zettabytes",
		"yottabytes",
		"ronnabytes",
		"quettabytes",
	];

	let units = match format {
		FormatType::Abbreviation => abbreviations,
		FormatType::FullName => full_names,
	};

	format_number_unit_f64(bytes, 1000.0, &units, 2, true)
}

pub fn format_size_ibibits(bytes: f64, format: FormatType) -> Result<String> {
	let abbreviations = [
		"b", "Kib", "Mib", "Gib", "Tib", "Pib", "Eib", "Zib", "Yib", "Rib", "Qib",
	];
	let full_names = [
		"bits",
		"kibibits",
		"mebibits",
		"gibibits",
		"tebibits",
		"pebibits",
		"exbibits",
		"zebibits",
		"yobibits",
		"robibits",
		"quebibits",
	];

	let units = match format {
		FormatType::Abbreviation => abbreviations,
		FormatType::FullName => full_names,
	};

	format_number_unit_f64(bytes * 8.0, 1024.0, &units, 2, true)
}

pub fn format_size_ibibytes(bytes: f64, format: FormatType) -> Result<String> {
	let abbreviations = [
		"B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB", "RiB", "QiB",
	];
	let full_names = [
		"bytes",
		"kibibytes",
		"mebibytes",
		"gibibytes",
		"tebibytes",
		"pebibytes",
		"exbibytes",
		"zebibytes",
		"yobibytes",
		"robibytes",
		"quebibytes",
	];

	let units = match format {
		FormatType::Abbreviation => abbreviations,
		FormatType::FullName => full_names,
	};

	format_number_unit_f64(bytes, 1024.0, &units, 2, true)
}
