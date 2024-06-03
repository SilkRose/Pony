use std::num::NonZeroUsize;

pub fn get_pages<T>(items: &[T], page_size: NonZeroUsize) -> Vec<&[T]> {
	items.chunks(page_size.get()).collect()
}

pub fn get_page<T>(items: &[T], page_size: NonZeroUsize, page: NonZeroUsize) -> Option<&[T]> {
	let items_start = page_size.get() * page.get();
	let items_end = page_size.get() * (page.get() + 1);

	if items_start >= items.len() {
		return None;
	}

	Some(if items_end > items.len() {
		&items[items_start..]
	} else {
		&items[items_start..items_end]
	})
}

pub fn get_pages_from_string(text: &str, page_size: NonZeroUsize) -> Vec<&str> {
	let page_size = page_size.get();
	let mut passed_newlines = 0;

	let checker = |char: char| {
		if char == '\n' {
			passed_newlines += 1;
			if passed_newlines == page_size {
				passed_newlines = 0;
				return true;
			}
		}
		false
	};

	text.split(checker).collect()
}

pub fn get_page_from_string(
	text: &str, page_size: NonZeroUsize, page: NonZeroUsize,
) -> Option<&str> {
	let pages = get_pages_from_string(text, page_size);
	pages.get(page.get()).copied()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn pages() {
		let list = [1, 2, 3, 4, 5, 6];
		let pages = get_pages(&list, NonZeroUsize::new(2).unwrap());
		assert_eq!(pages, vec![[1, 2], [3, 4], [5, 6]]);
	}
	#[test]
	fn page() {
		let list = [1, 2, 3, 4, 5, 6];
		let pages = get_page(
			&list,
			NonZeroUsize::new(2).unwrap(),
			NonZeroUsize::new(2).unwrap(),
		)
		.unwrap();
		assert_eq!(pages, vec![5, 6]);
	}
}
