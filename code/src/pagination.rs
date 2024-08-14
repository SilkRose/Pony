use std::num::NonZeroUsize;

pub fn get_pages<T>(items: &[T], page_size: NonZeroUsize) -> Vec<&[T]> {
	items.chunks(page_size.get()).collect()
}

pub fn get_page<T>(items: &[T], page_size: NonZeroUsize, page: NonZeroUsize) -> Option<&[T]> {
	let items_start = page_size.get() * (page.get() - 1);
	let items_end = page_size.get() * page.get();

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
	pages.get(page.get() - 1).copied()
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn pages() {
		let list = [1, 2, 3, 4, 5, 6];
		let pages = get_pages(&list, NonZeroUsize::new(2).unwrap());
		assert_eq!(vec![[1, 2], [3, 4], [5, 6]], pages);
	}
	#[test]
	fn page() {
		let list = [1, 2, 3, 4, 5, 6];
		let page = get_page(
			&list,
			NonZeroUsize::new(2).unwrap(),
			NonZeroUsize::new(2).unwrap(),
		)
		.unwrap();
		assert_eq!(vec![3, 4], page);
	}
	#[test]
	fn page_overflow() {
		let list = [1, 2, 3, 4, 5, 6];
		let page = get_page(
			&list,
			NonZeroUsize::new(1).unwrap(),
			NonZeroUsize::new(7).unwrap(),
		);
		assert_eq!(None, page);
	}
	#[test]
	fn page_under_sized() {
		let list = [1, 2, 3, 4, 5, 6];
		let page = get_page(
			&list,
			NonZeroUsize::new(8).unwrap(),
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap();
		assert_eq!(vec![1, 2, 3, 4, 5, 6], page);
	}
	#[test]
	fn pages_string() {
		let string = "12\n34\n56";
		let pages = get_pages_from_string(string, NonZeroUsize::new(2).unwrap());
		assert_eq!(vec!["12\n34", "56"], pages);
	}
	#[test]
	fn page_string() {
		let string = "12\n34\n56";
		let page = get_page_from_string(
			string,
			NonZeroUsize::new(2).unwrap(),
			NonZeroUsize::new(2).unwrap(),
		)
		.unwrap();
		assert_eq!("56", page);
	}
	#[test]
	fn page_string_overflow() {
		let string = "12\n34\n56";
		let page = get_page_from_string(
			string,
			NonZeroUsize::new(3).unwrap(),
			NonZeroUsize::new(2).unwrap(),
		);
		assert_eq!(None, page);
	}
	#[test]
	fn page_string_under_sized() {
		let string = "12\n34\n56";
		let page = get_page_from_string(
			string,
			NonZeroUsize::new(8).unwrap(),
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap();
		assert_eq!("12\n34\n56", page);
	}
}
