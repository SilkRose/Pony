//#![deny(missing_docs)]
//#![doc = include_str!("../readme.md")]

use std::cmp::Ordering;

pub trait Vector<T: Ord> {
	fn sort_vec(self) -> Vec<T>;
	fn dedup_vec(self) -> Vec<T>;
	fn sort_and_dedup_vec(self) -> Vec<T>
	where
		Self: Sized,
	{
		self.sort_vec().dedup_vec()
	}
	fn extend_vec(self, vec: Vec<T>) -> Vec<T>;
}

impl<T: Ord> Vector<T> for Vec<T> {
	fn sort_vec(mut self) -> Vec<T> {
		self.sort();
		self
	}
	fn dedup_vec(mut self) -> Vec<T> {
		self.dedup();
		self
	}
	fn extend_vec(mut self, vec: Vec<T>) -> Vec<T> {
		self.extend(vec);
		self
	}
}

pub fn compare<T: PartialOrd>(a: &T, b: &T) -> Ordering {
	if a < b {
		Ordering::Less
	} else if a > b {
		Ordering::Greater
	} else {
		Ordering::Equal
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn sort() {
		let vec = vec![2, 1, 1, 2, 0, 3].sort_vec();
		assert_eq!(vec, vec![0, 1, 1, 2, 2, 3]);
	}
	#[test]
	fn dedup() {
		let vec = vec![2, 1, 1, 0, 0, 3].dedup_vec();
		assert_eq!(vec, vec![2, 1, 0, 3]);
	}
	#[test]
	fn sort_and_dedup() {
		let vec = vec![2, 1, 1, 0, 0, 3].sort_and_dedup_vec();
		assert_eq!(vec, vec![0, 1, 2, 3]);
	}
	#[test]
	fn extend() {
		let vec = vec![2, 1, 1, 0, 0, 3].extend_vec(vec![4, 5]);
		assert_eq!(vec, vec![2, 1, 1, 0, 0, 3, 4, 5]);
	}
	#[test]
	fn compare() {
		let mut list = vec![(3, 1), (2, 2), (1, 3)];
		list.sort_by(|a, b| super::compare(&a.0, &b.0));
		assert_eq!(list, vec![(1, 3), (2, 2), (3, 1)]);
	}
}
