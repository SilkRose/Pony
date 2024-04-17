//#![deny(missing_docs)]
//#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
/// Parser module for library use.
pub mod parser;

use regex::Regex;

pub struct Options {
	remove_apostrophe: bool,
}

pub struct Stats {
	total_chars: usize,
	text_chars: usize,
	letter_chars: usize,
	md_syntax_chars: usize,
	punctuation_chars: usize,
	total_words: usize,
	unique_words: usize,
	sentence_lengths: Vec<usize>,
	total_sentences: usize,
	paragraph_lengths: Vec<usize>,
	total_paragraphs: usize,
	total_pages: usize,
	complex_words: usize,
	simple_words: usize,
	reading_time: usize,
	words: Vec<Word>,
	search_words: Option<Vec<WordStat>>,
}

pub struct Word {
	text: String,
	count: usize,
	indexes: Vec<usize>,
}

pub struct WordStat {
	text: String,
	regex: String,
	count: usize,
	indexes: Vec<usize>,
}

pub fn word_count(text: String) -> usize {
	let plain_text = parser::parse(text);
	remove_punctuation(plain_text).split_whitespace().count()
}

fn remove_punctuation(text: String) -> String {
	let re = Regex::new(r"\'([tsd]\b|ve\b|ll\b|re\b)").unwrap();
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s]").unwrap();
	re.replace_all(&text, "").into()
}
