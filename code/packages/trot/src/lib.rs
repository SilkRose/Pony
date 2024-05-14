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
