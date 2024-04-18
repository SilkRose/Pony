use crate::{parser, text_stats::TextType};
use parser::parse;
use regex::Regex;

#[derive(Debug)]
pub struct SearchWords {
	identifier: String,
	regex: Regex,
}

#[derive(Debug)]
pub struct WordResult {
	identifier: String,
	count: usize,
}

pub fn get_word_stats(
	text: String, text_type: TextType, words: Vec<SearchWords>,
) -> Vec<WordResult> {
	let text = match text_type {
		TextType::Markdown => parse(text),
		TextType::Plaintext => text,
	};
	words
		.iter()
		.map(|word| WordResult {
			identifier: word.identifier.clone(),
			count: word.regex.find_iter(&text).count(),
		})
		.collect()
}
