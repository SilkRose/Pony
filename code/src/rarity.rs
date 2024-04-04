pub fn count_occurances(text: String, word: String) -> usize {
	text.split_whitespace().filter(|w| w == &word).count()
}
