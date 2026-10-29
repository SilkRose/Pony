/// Comma delineates numbers. Kept private for use only in this module.
fn format_number(number: &str) -> String {
	number
		.as_bytes()
		.rchunks(3)
		.rev()
		.map(|c| unsafe { std::str::from_utf8_unchecked(c) })
		.collect::<Vec<&str>>()
		.join(",")
}

macro_rules! format_number_float {
	($fun:ident, $T:ty, $early_fun:ident, $early_cast:ty) => {
		pub fn $fun(number: $T, decimal_places: usize) -> String {
			if decimal_places == 0 {
				return $early_fun(number as $early_cast);
			}
			let number_str = format!("{number:.decimal_places$}");
			let (integer, fraction) = number_str.split_once(".").expect("Will always be present.");
			match number < 0.0 {
				false => format!("{}.{fraction}", format_number(integer)),
				true => format!(
					"-{}.{fraction}",
					format_number(integer.trim_start_matches("-"))
				),
			}
		}
	};
}

format_number_float!(format_number_f32, f32, format_number_i32, i32);
format_number_float!(format_number_f64, f64, format_number_i64, i64);

macro_rules! format_number_signed {
	($fun:ident, $T:ty) => {
		pub fn $fun(number: $T) -> String {
			match number < 0 {
				false => format_number(&number.to_string()),
				true => format!("-{}", format_number(&number.abs().to_string())),
			}
		}
	};
}

format_number_signed!(format_number_i8, i8);
format_number_signed!(format_number_i16, i16);
format_number_signed!(format_number_i32, i32);
format_number_signed!(format_number_i64, i64);
format_number_signed!(format_number_i128, i128);
format_number_signed!(format_number_isize, isize);

macro_rules! format_number_unsigned {
	($fun:ident, $T:ty) => {
		pub fn $fun(number: $T) -> String {
			format_number(&number.to_string())
		}
	};
}

format_number_unsigned!(format_number_u8, u8);
format_number_unsigned!(format_number_u16, u16);
format_number_unsigned!(format_number_u32, u32);
format_number_unsigned!(format_number_u64, u64);
format_number_unsigned!(format_number_u128, u128);
format_number_unsigned!(format_number_usize, usize);

pub enum DecimalHandling {
	Truncate,
	Round,
	RoundUp,
	RoundDown,
	RoundTiesEven,
}

macro_rules! format_number_unit_float {
	($fun:ident, $T:ty, $format_fun:ident) => {
		pub fn $fun(
			number: $T, divisor: $T, units: &[&str], decimal_places: usize,
			decimal_handling: Option<DecimalHandling>, spaced: bool, trim_tail: bool,
		) -> String {
			if units.is_empty() {
				return $format_fun(number, decimal_places);
			}
			let space = if spaced { " " } else { "" };
			let mut current = number;
			let mut current_unit = units.first().expect("Will always be present.");
			let mut loop_broken = false;
			for unit in units.iter() {
				if current < divisor {
					loop_broken = true;
					break;
				}
				current /= divisor;
				current_unit = unit;
			}
			if let Some(handler) = decimal_handling {
				current = match handler {
					DecimalHandling::Truncate => current.trunc(),
					DecimalHandling::Round => current.round(),
					DecimalHandling::RoundUp => current.ceil(),
					DecimalHandling::RoundDown => current.floor(),
					DecimalHandling::RoundTiesEven => current.round_ties_even(),
				}
			}
			let mut number_str = match loop_broken {
				true => format!("{current:.decimal_places$}"),
				false => $format_fun(current, decimal_places),
			};
			if trim_tail && number_str.contains(".") {
				number_str = number_str
					.trim_end_matches("0")
					.trim_end_matches(".")
					.to_string()
			}
			format!("{number_str}{space}{current_unit}")
		}
	};
}

format_number_unit_float!(format_number_unit_f32, f32, format_number_f32);
format_number_unit_float!(format_number_unit_f64, f64, format_number_f64);

macro_rules! format_number_unit_integer {
	($fun:ident, $T:ty, $format_fun:ident) => {
		pub fn $fun(number: $T, divisor: $T, units: &[&str], spaced: bool) -> String {
			if units.is_empty() {
				return $format_fun(number);
			}
			let space = if spaced { " " } else { "" };
			let mut current = number;
			let mut current_unit = units.first().expect("Will always be present.");
			for unit in units.iter() {
				if current < divisor {
					return format!("{current}{space}{unit}");
				}
				current /= divisor;
				current_unit = unit;
			}
			format!("{}{space}{current_unit}", $format_fun(current))
		}
	};
}

format_number_unit_integer!(format_number_unit_i8, i8, format_number_i8);
format_number_unit_integer!(format_number_unit_i16, i16, format_number_i16);
format_number_unit_integer!(format_number_unit_i32, i32, format_number_i32);
format_number_unit_integer!(format_number_unit_i64, i64, format_number_i64);
format_number_unit_integer!(format_number_unit_i128, i128, format_number_i128);
format_number_unit_integer!(format_number_unit_isize, isize, format_number_isize);

format_number_unit_integer!(format_number_unit_u8, u8, format_number_u8);
format_number_unit_integer!(format_number_unit_u16, u16, format_number_u16);
format_number_unit_integer!(format_number_unit_u32, u32, format_number_u32);
format_number_unit_integer!(format_number_unit_u64, u64, format_number_u64);
format_number_unit_integer!(format_number_unit_u128, u128, format_number_u128);
format_number_unit_integer!(format_number_unit_usize, usize, format_number_usize);

pub enum FormatType {
	MetricPrefix,
	ShortScaleName,
}

pub fn format_number_unit_metric(
	number: f64, format: FormatType, decimal_places: usize, trim_tail: bool,
) -> String {
	if number <= 1000.0 {
		return number.to_string();
	}
	let metric_prefixes = ["K", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
	let short_scale_names = [
		"thousand",
		"million",
		"billion",
		"trillion",
		"quadrillion",
		"quintillion",
		"sextillion",
		"septillion",
		"octillion",
		"nonillion",
	];
	let (units, spaced) = match format {
		FormatType::MetricPrefix => (metric_prefixes, false),
		FormatType::ShortScaleName => (short_scale_names, true),
	};
	format_number_unit_f64(
		number / 1000.0,
		1000.0,
		&units,
		decimal_places,
		None,
		spaced,
		trim_tail,
	)
}
