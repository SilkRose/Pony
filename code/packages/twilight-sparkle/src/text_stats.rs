use crate::parser;
use parser::parse;
use regex::Regex;

pub enum TextType {
	Markdown,
	Plaintext,
}

pub fn word_count(text: String) -> usize {
	let plain_text = parser::parse(text);
	remove_punctuation(plain_text).split_whitespace().count()
}

pub fn remove_punctuation(text: String) -> String {
	let re = Regex::new(r"!([\'’]([tsd]\b|ve\b|ll\b|re\b))").unwrap();
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s\'’]").unwrap();
	re.replace_all(&text, "").into()
}
