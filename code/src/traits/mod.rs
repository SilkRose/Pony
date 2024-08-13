#![deny(missing_docs)]
#![doc = include_str!("./readme.md")]

use std::cmp::Ordering;

/// Base trait for Vec operations.
pub trait BasicVector<T> {
	/// Extends a Vec and returns it.
	fn extend_vec(self, vec: Vec<T>) -> Vec<T>;
	/// Reverse a Vec and returns it.
	fn reverse_vec(self) -> Vec<T>;
}

/// Trait for Vec operations that require Ord.
pub trait OrderedVector<T: Ord>: BasicVector<T> {
	/// Sorts a Vec and returns it.
	fn sort_vec(self) -> Vec<T>;
	/// Dedupes a Vec and returns it.
	fn dedup_vec(self) -> Vec<T>;
	/// Sorts and dedupes a Vec then returns it.
	fn sort_and_dedup_vec(self) -> Vec<T>;
}

impl<T> BasicVector<T> for Vec<T> {
	fn extend_vec(mut self, vec: Vec<T>) -> Vec<T> {
		self.extend(vec);
		self
	}
	fn reverse_vec(mut self) -> Vec<T> {
		self.reverse();
		self
	}
}

impl<T: Ord> OrderedVector<T> for Vec<T> {
	fn sort_vec(mut self) -> Vec<T> {
		self.sort();
		self
	}
	fn dedup_vec(mut self) -> Vec<T> {
		self.dedup();
		self
	}
	fn sort_and_dedup_vec(mut self) -> Vec<T> {
		self.sort();
		self.dedup();
		self
	}
}

/// Compare elements for a sorting function.
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
	fn reverse() {
		let vec = vec![2, 1, 1, 2, 0, 3].reverse_vec();
		assert_eq!(vec, vec![3, 0, 2, 1, 1, 2]);
	}
	#[test]
	fn compare_less() {
		let mut list = vec![(3, 1), (2, 2), (1, 3)];
		list.sort_by(|a, b| super::compare(&a.0, &b.0));
		assert_eq!(list, vec![(1, 3), (2, 2), (3, 1)]);
	}
	#[test]
	fn compare_greater() {
		let mut list = vec![(1, 1), (2, 2), (3, 3)];
		list.sort_by(|a, b| super::compare(&a.0, &b.0));
		assert_eq!(list, vec![(1, 1), (2, 2), (3, 3)]);
	}
	#[test]
	fn compare_equel() {
		let mut list = vec![(1, 1), (1, 2), (1, 3)];
		list.sort_by(|a, b| super::compare(&a.0, &b.0));
		assert_eq!(list, vec![(1, 1), (1, 2), (1, 3)]);
	}
}
