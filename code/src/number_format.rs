use super::error::Result;

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
