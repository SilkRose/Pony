use super::error::Result;
use super::md_to_plaintext::parse;
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
) -> Result<Vec<WordResult>> {
	let text = match options.replace_hyphen {
		true => text.replace('-', " "),
		false => text,
	};
	let text = match options.remove_apostrophe {
		true => text.replace(['\'', '’'], ""),
		false => text,
	};
	let text = match options.remove_punctuation {
		true => remove_punctuation(text)?,
		false => text,
	};
	println!("{text}");
	let words = words
		.iter()
		.map(|word| WordResult {
			identifier: word.identifier.clone(),
			count: word.regex.find_iter(&text).count(),
		})
		.collect();
	Ok(words)
}

pub fn word_count(text: String) -> Result<usize> {
	let plain_text = parse(text);
	Ok(remove_punctuation(plain_text)?.split_whitespace().count())
}

pub fn remove_punctuation(text: String) -> Result<String> {
	let re = Regex::new(r"!([\'’]([tsd]\b|ve\b|ll\b|re\b))")?;
	let text = re.replace_all(&text, "");
	let re = Regex::new(r"[^\w\s\'’]")?;
	Ok(re.replace_all(&text, "").into())
}
