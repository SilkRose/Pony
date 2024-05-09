use std::{num::NonZeroUsize, slice};

fn get_pages<T>(items: &[T], page_size: NonZeroUsize) -> Vec<&[T]> {
	items.chunks(page_size.get()).collect()
}

fn get_page<T>(items: &[T], page_size: NonZeroUsize, page: NonZeroUsize) -> Option<&[T]> {
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

fn get_page_with_unsafe_code<T>(
	items: &[T], page_size: NonZeroUsize, page: NonZeroUsize,
) -> Option<&[T]> {
	let items_start = page_size.get() * page.get();

	if items_start >= items.len() {
		return None;
	}

	let num_items = usize::min(page_size.get(), items.len() - items_start);
	unsafe {
		let ptr = items as *const [T] as *const T;
		let slice = slice::from_raw_parts(ptr.add(items_start), num_items);
		Some(slice)
	}
}
