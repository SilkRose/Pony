use crate::{parser, text_stats::TextType};
use parser::parse;
use regex::Regex;

pub fn get_word_stats(text: String, text_type: TextType) {
	let text = match text_type {
		TextType::Markdown => parse(text),
		TextType::Plaintext => text,
	};
}

pub struct SearchWords {
	identifier: String,
	regex: Regex,
}

pub struct Word {
	mane_word: String,
	word_regex: String,
	occurances: usize,
}

pub fn count_regex(text: String, word: String, regex: String) -> Word {
	let word_regex = Regex::new(&regex).unwrap();
	let matches = word_regex.find_iter(&text);
	Word {
		mane_word: word,
		word_regex: regex,
		occurances: word_regex.find_iter(&text).count(),
	}
}
