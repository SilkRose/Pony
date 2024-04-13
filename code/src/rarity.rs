use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
	mane_word: String,
	word_regex: String,
	occurances: usize,
	percentage: usize,
	total_chars: usize,
	char_percentage: usize,
}

pub fn count_occurances(text: String, word: String) -> usize {
	text.split_whitespace().filter(|w| w == &word).count()
}

pub fn count_regex(text: String, word: String, regex: String) -> Word {
	let word_regex = Regex::new(&regex).unwrap();
	let matches = word_regex.find_iter(&text);
	let chars = matches.map(|m| m.as_str()).collect::<Vec<_>>().join("").chars().count();
	Word {
		mane_word: word,
		word_regex: regex,
		occurances: word_regex.find_iter(&text).count(),
		percentage: 0,
		total_chars: chars,
		char_percentage: 0,
	}
}
