use super::markdown::plaintext::parse;
use regex::Regex;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub fn count_matches(text: &str, includes: Regex) -> usize {
	includes.find_iter(text).count()
}

pub fn word_count(text: &str) -> Result<usize> {
	let plain_text = parse(text);
	Ok(remove_punctuation(plain_text)?.split_whitespace().count())
}

pub fn remove_punctuation(text: String) -> Result<String> {
	let re = Regex::new(r"!([\'’]([tsd]\b|ve\b|ll\b|re\b))")?;
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s\'’]")?;
	Ok(re.replace_all(&text, "").into())
}

#[cfg(test)]
mod tests {
	use super::*;
	use regex::Regex;

	#[test]
	fn word_counter() {
		let text = "Pinkie Pie is best pony!";
		let words = word_count(text).unwrap();
		assert_eq!(5, words);
	}

	#[test]
	fn match_counter() {
		let text = "Pinkie Pie is best pony!";
		let includes = Regex::new(r"Pinkie Pie").unwrap();
		let count = count_matches(text, includes);
		assert_eq!(1, count);
	}
}
