//#![deny(missing_docs)]
//#![doc = include_str!("../readme.md")]
#![allow(dead_code)]
/// Parser module for library use.
pub mod parser;

use hyphenation::{Hyphenator, Language, Load, Standard};
use regex::Regex;

pub struct Options {
	remove_apostrophe: bool,
	remove_punctuation: bool,
	page_style: PageCount,
	reading_time: ReadingTime,
	short_word_max: usize,
	long_word_min: usize,
	comma: PunctuationType,
	single_quote: PunctuationType,
	double_quote: PunctuationType,
	ellipses: PunctuationType,
	hyphen: PunctuationType,
	en_dash: PunctuationType,
	em_dash: PunctuationType,
	period: PunctuationType,
	exclamation_mark: PunctuationType,
	question_mark: PunctuationType,
	colon: PunctuationType,
	semicolon: PunctuationType,
	paranthesis: PunctuationType,
}

pub enum PunctuationType {
	ClauseSeparator,
	ClauseContainer,
	ClauseTerminator,
	DynamicDetection,
	SkipOver,
}

pub enum PageCount {
	WordsPerPage(usize),
	CharPerPAge(usize),
	CharPageSize((usize, usize)),
}

pub enum ReadingTime {
	WordsPerMin(usize),
	MsPerChar(usize),
	MsPerSyllable(usize),
}

pub struct Stats {
	total_chars: usize,
	text_chars: usize,
	letter_chars: usize,
	md_syntax_chars: usize,
	punctuation_chars: usize,
	syllable_count: usize,
	syllables_per_word: Vec<usize>,
	word_lengths_chars: Vec<usize>,
	word_lengths_letters: Vec<usize>,
	total_words: usize,
	unique_words: usize,
	clause_lengths: Vec<usize>,
	sentence_lengths: Vec<usize>,
	total_sentences: usize,
	paragraph_lengths: Vec<usize>,
	total_paragraphs: usize,
	total_pages: usize,
	long_words: usize,
	medium_words: usize,
	short_words: usize,
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
	let en_us = Standard::from_embedded(Language::EnglishUS).unwrap();
	let hyphenated: Vec<&str> = en_us
		.hyphenate("Fluttershy")
		.into_iter()
		.segments()
		.collect();
	println!("{:?}", hyphenated);
	remove_punctuation(plain_text).split_whitespace().count()
}

fn remove_punctuation(text: String) -> String {
	let re = Regex::new(r"\'([tsd]\b|ve\b|ll\b|re\b)").unwrap();
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s]").unwrap();
	re.replace_all(&text, "").into()
}
