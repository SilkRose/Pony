//#![deny(missing_docs)]
//#![doc = include_str!("../readme.md")]
#![allow(dead_code, unused_imports, unused_mut, unused_variables)]
/// Parser module for library use.
pub mod parser;

use hyphenation::{Hyphenator, Language, Load, Standard};
use parser::parse;
use regex::Regex;

pub struct StatOptions {
	text_type: TextType,
	easy_words: Option<Vec<String>>,
	paragraph_separator: ParagraphSeparator,
	page_style: PageCount,
	reading_time: ReadingTime,
	word_borders: WordBorders,
	syllable: SyllableCount,
	ellipses: PunctuationType,
	hyphen: PunctuationType,
	en_dash: PunctuationType,
	em_dash: PunctuationType,
}

pub enum TextType {
	Markdown,
	Plaintext,
}

pub enum ParagraphSeparator {
	NewLine,
	DoubleNewLine,
	NewLineTab,
}

pub enum PageCount {
	WordsPerPage(usize),
	CharPerPage(usize),
	CharPageSize((usize, usize)),
}

pub enum ReadingTime {
	WordsPerMin(usize),
	MsPerChar(usize),
	MsPerSyllable(usize),
}

pub enum WordBorders {
	Chars((usize, usize)),
	Syllables((usize, usize)),
}

pub enum SyllableCount {
	Standard,
	Extended((usize, usize)),
}

pub enum PunctuationType {
	ClauseSeparator,
	SentenceSeparator,
	DynamicDetect(Option<usize>),
	Whitespace,
	Ignore,
}

pub struct Stats {
	char_count: usize,
	chars_per_word: Vec<usize>,
	syllable_count: usize,
	syllables_per_word: Vec<usize>,
	word_count: usize,
	unique_word_count: usize,
	clause_count: usize,
	chars_per_clause: Vec<usize>,
	words_per_clause: Vec<usize>,
	sentence_count: usize,
	words_per_sentence: Vec<usize>,
	paragraph_count: usize,
	words_per_paragraph: Vec<usize>,
	total_pages: usize,
	long_words: usize,
	medium_words: usize,
	short_words: usize,
	reading_time: usize,
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

pub fn get_stats(text: String, options: StatOptions) {
	let text = match options.text_type {
		TextType::Markdown => parse(text),
		TextType::Plaintext => text,
	};
	let paragraph_separator = match options.paragraph_separator {
		ParagraphSeparator::NewLine => "\n",
		ParagraphSeparator::DoubleNewLine => "\n\n",
		ParagraphSeparator::NewLineTab => "\n\t",
	};
	let chars = text.chars().count();
	let paragraphs = text.split(paragraph_separator).collect::<Vec<_>>();
}
