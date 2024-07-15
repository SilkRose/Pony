use super::error::Result;
use super::markdown::md_to_plaintext::parse;
use regex::Regex;

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
