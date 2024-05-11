use super::text_stats::remove_punctuation;
use regex::Regex;

pub struct WordOptions {
	pub replace_hyphen: bool,
	pub remove_apostrophe: bool,
	pub remove_punctuation: bool,
}

#[derive(Debug)]
pub struct SearchWords {
	pub identifier: String,
	pub regex: Regex,
}

#[derive(Debug)]
pub struct WordResult {
	identifier: String,
	count: usize,
}

pub fn get_word_stats(
	text: String, options: WordOptions, words: Vec<SearchWords>,
) -> Vec<WordResult> {
	let text = match options.replace_hyphen {
		true => text.replace('-', " "),
		false => text,
	};
	let text = match options.remove_apostrophe {
		true => text.replace(['\'', '’'], ""),
		false => text,
	};
	let text = match options.remove_punctuation {
		true => remove_punctuation(text),
		false => text,
	};
	println!("{text}");
	words
		.iter()
		.map(|word| WordResult {
			identifier: word.identifier.clone(),
			count: word.regex.find_iter(&text).count(),
		})
		.collect()
}
