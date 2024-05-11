use super::md_to_plaintext::parse;
use regex::Regex;

pub fn word_count(text: String) -> usize {
	let plain_text = parse(text);
	remove_punctuation(plain_text).split_whitespace().count()
}

pub fn remove_punctuation(text: String) -> String {
	let re = Regex::new(r"!([\'’]([tsd]\b|ve\b|ll\b|re\b))").unwrap();
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s\'’]").unwrap();
	re.replace_all(&text, "").into()
}
