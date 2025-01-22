use regex::Regex;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub enum State {
	Text,
	OpeningTag,
	ClosingTag,
	SingleTag,
}

pub enum TagType {
	Opening,
	Closing,
	Single,
}

pub enum TokenKind {
	Text,
	ParagraphBreak,
	HeadingOpen,
	HeadingClose,
	// ...
}

pub struct Token {
	pub kind: TokenKind,
	pub value: String,
	pub start_position: usize,
	pub end_position: usize,
}

pub struct Tokenizer {
	pub source: String,
	pub search_source: String,
	pub tokens: Vec<Token>,
}

pub fn tokenize_peekable(source: &str) -> Result<Vec<Token>> {
	let mut chars = source.chars();
	let mut tokens = Vec::new();
	while let Some(char) = chars.next() {
		if char == '[' {
			let next = chars.next();
			if Some('h') == next && Some('1') == chars.next() && Some(']') == chars.next() {
				//println!("h1 open");
			} else if Some('/') == next
				&& Some('h') == chars.next()
				&& Some('1') == chars.next()
				&& Some(']') == chars.next()
			{
				//println!("h1 close");
			} else if Some('i') == next && Some(']') == chars.next() {
				//println!("i open");
			} else if Some('/') == next && Some('i') == chars.next() && Some(']') == chars.next() {
				//println!("i close");
			}
		}
	}

	Ok(tokens)
}

pub fn tokenize_regex(
	source: &str, h1o: &Regex, h1c: &Regex, io: &Regex, ic: &Regex,
) -> Result<Vec<Token>> {
	let mut tokens = Vec::new();
	for (i, c) in source.char_indices() {
		if c == '[' {
			let sub = &source[i..];
			if h1o.is_match(sub) {
				//println!("h1 open");
			} else if h1c.is_match(sub) {
				//println!("h1 close");
			} else if io.is_match(sub) {
				//println!("i open");
			} else if ic.is_match(sub) {
				//println!("i close");
			}
		}
	}

	Ok(tokens)
}

pub fn tokenize_substring(source: &str) -> Result<Vec<Token>> {
	let mut tokens = Vec::new();
	for (i, c) in source.char_indices() {
		if c == '[' {
			let sub = &source[i..];
			if sub.starts_with("[I]") || sub.starts_with("i]") {
				//println!("i open");
			} else if sub.starts_with("[/I]") || sub.starts_with("/i]") {
				//println!("i close");
			} else if sub.starts_with("[h1]") || sub.starts_with("H1]") {
				//println!("h1 open");
			} else if sub.starts_with("[/h1]") || sub.starts_with("/H1]") {
				//println!("h1 close");
			}
		}
	}

	Ok(tokens)
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::*;
	use crate::time::{format_milliseconds, unix_time};

	#[test]
	fn parse_bbcode() -> Result<()> {
		//let bbcode = "[h1]Hi[/h1]\n[i]Pinkie Pie![/i]";
		let bbcode =
			fs::read_to_string("./publish/stories/pink-mended-sparkles/pink-mended-sparkles.txt")?;
		let start = unix_time()?;
		for _ in 1..=1_000_000 {
			tokenize_peekable(&bbcode)?;
		}
		let end = unix_time()?;
		let time = format_milliseconds(end - start, None)?;
		println!("Time (peek): {time}");

		let h1o = Regex::new(r"^\[[hH]1\]").unwrap();
		let h1c = Regex::new(r"^\[\/[hH]1\]").unwrap();
		let io = Regex::new(r"^\[[iI]\]").unwrap();
		let ic = Regex::new(r"^\[\/[iI]\]").unwrap();

		let start = unix_time()?;
		for _ in 1..=1_000_000 {
			tokenize_regex(&bbcode, &h1o, &h1c, &io, &ic)?;
		}
		let end = unix_time()?;
		let time = format_milliseconds(end - start, None)?;
		println!("Time (regex): {time}");

		let start = unix_time()?;
		for _ in 1..=1_000_000 {
			tokenize_substring(&bbcode)?;
		}
		let end = unix_time()?;
		let time = format_milliseconds(end - start, None)?;
		println!("Time (sub): {time}");

		Ok(())
	}
}
