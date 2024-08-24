type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub fn format_number_f64(number: f64, decimal_places: usize) -> Result<String> {
	let number = format!("{number:.decimal_places$}");
	let decimal = number.split('.').last().unwrap();
	let number = number.split('.').collect::<Vec<_>>();
	let number = number.first().unwrap();
	let (number, negative) = match number.starts_with('-') {
		true => (number.trim_start_matches('-').to_string(), true),
		false => (number.to_string(), false),
	};
	let number = format_number(number)?;
	match negative {
		true => Ok(format!("-{number}.{decimal}")),
		false => Ok(format!("{number}.{decimal}")),
	}
}

pub fn format_number_i128(number: i128) -> Result<String> {
	let (number, negative) = match number < 0 {
		true => (number.abs().to_string(), true),
		false => (number.to_string(), false),
	};
	let number = format_number(number)?;
	match negative {
		true => Ok(format!("-{number}")),
		false => Ok(number.to_string()),
	}
}

pub fn format_number_u128(number: u128) -> Result<String> {
	format_number(number.to_string())
}

fn format_number(number: String) -> Result<String> {
	Ok(number
		.as_bytes()
		.rchunks(3)
		.rev()
		.map(std::str::from_utf8)
		.collect::<Result<Vec<&str>, _>>()?
		.join(","))
}

pub fn format_number_unit_f64(
	number: f64, divisor: f64, units: &[&str], decimal_places: usize, spaced: bool,
) -> Result<String> {
	let spaced = if spaced { " " } else { "" };
	let mut current = number;
	for unit in units.iter() {
		if current < divisor {
			return Ok(format!("{:.1$}{spaced}{unit}", current, decimal_places));
		}
		current /= divisor;
	}
	Ok(format!(
		"{}{spaced}{}",
		format_number_f64(current, decimal_places)?,
		units.last().unwrap()
	))
}

pub fn format_number_unit_u128(
	number: u128, divisor: u128, units: &[&str], spaced: bool,
) -> Result<String> {
	let spaced = if spaced { " " } else { "" };
	let mut current = number;
	for unit in units.iter() {
		if current < divisor {
			return Ok(format!("{current}{spaced}{unit}"));
		}
		current /= divisor;
	}
	Ok(format!(
		"{}{spaced}{}",
		format_number_u128(current)?,
		units.last().unwrap()
	))
}

pub fn format_number_unit_i128(
	number: i128, divisor: i128, units: &[&str], spaced: bool,
) -> Result<String> {
	let spaced = if spaced { " " } else { "" };
	let mut current = number;
	for unit in units.iter() {
		if current < divisor {
			return Ok(format!("{current}{spaced}{unit}"));
		}
		current /= divisor;
	}
	Ok(format!(
		"{}{spaced}{}",
		format_number_i128(current)?,
		units.last().unwrap()
	))
}

pub enum FormatType {
	MetricPrefix,
	ShortScaleName,
}

pub fn format_number_unit_metric(
	number: f64, format: FormatType, decimal_places: usize,
) -> Result<String> {
	let metrix_prefixes = ["K", "M", "G", "T", "P", "E", "Z", "Y", "R", "Q"];
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
		FormatType::MetricPrefix => (metrix_prefixes, false),
		FormatType::ShortScaleName => (short_scale_names, true),
	};
	format_number_unit_f64(number, 1000.0, &units, decimal_places, spaced)
}
