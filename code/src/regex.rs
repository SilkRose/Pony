use regex::Regex;

pub fn matches(string: &str, includes: &Option<Regex>, excludes: &Option<Regex>) -> bool {
	if let Some(excludes) = excludes {
		if excludes.is_match(string) {
			return false;
		}
	}
	if let Some(includes) = includes {
		return includes.is_match(string);
	}
	true
}

#[cfg(test)]
mod tests {
	use super::*;
	use regex::Regex;

	#[test]
	fn none() {
		let matched = matches("Pinkie Pie is best pony!", &None, &None);
		assert!(matched);
	}
	#[test]
	fn include_match() {
		let includes = Some(Regex::new("Pinkie Pie").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &includes, &None);
		assert!(matched);
	}
	#[test]
	fn include_miss() {
		let includes = Some(Regex::new("Rarity").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &includes, &None);
		assert!(!matched);
	}
	#[test]
	fn exclude_match() {
		let excludes = Some(Regex::new("Pinkie").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &None, &excludes);
		assert!(!matched);
	}
	#[test]
	fn exclude_miss() {
		let excludes = Some(Regex::new("Rarity").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &None, &excludes);
		assert!(matched);
	}
	#[test]
	fn include_match_exclude_miss() {
		let includes = Some(Regex::new("Pinkie Pie").unwrap());
		let excludes = Some(Regex::new("Rarity").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &includes, &excludes);
		assert!(matched);
	}
	#[test]
	fn include_miss_exclude_match() {
		let includes = Some(Regex::new("Rarity").unwrap());
		let excludes = Some(Regex::new("Pinkie Pie").unwrap());
		let matched = matches("Pinkie Pie is best pony!", &includes, &excludes);
		assert!(!matched);
	}
}
