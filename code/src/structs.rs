use serde::de::Error;
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;

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

pub fn option_number<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<Cow<str>> = Option::deserialize(deserializer)?;
	s.filter(|s| !s.is_empty())
		.map(|s| s.parse::<i32>().map_err(D::Error::custom))
		.transpose()
}

pub fn option_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
	D: Deserializer<'de>,
{
	let s: Option<String> = Option::deserialize(deserializer)?;
	Ok(s.filter(|s| !s.is_empty()))
}
