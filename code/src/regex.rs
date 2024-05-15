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
