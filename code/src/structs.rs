#[macro_export]
macro_rules! option_sort {
	($T:ty, $opt:ident, $id:ident) => {
		impl Ord for $T {
			fn cmp(&self, other: &Self) -> Ordering {
				(self.$opt.is_none(), self.$opt, self.$id).cmp(&(
					other.$opt.is_none(),
					other.$opt,
					other.$id,
				))
			}
		}

		impl PartialOrd for $T {
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Some(self.cmp(other))
			}
		}

		impl PartialEq for $T {
			fn eq(&self, other: &Self) -> bool {
				self.$id == other.$id
			}
		}

		impl Eq for $T {}
	};
}
