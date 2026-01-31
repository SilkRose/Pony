use std::collections::HashMap;
use std::hash::RandomState;

#[derive(Default)]
pub struct AliasedHashMap<K, V, S = RandomState> {
	pub data: HashMap<K, V, S>,
	pub aliases: HashMap<K, K, S>,
}

impl<K, V> From<HashMap<K, V>> for AliasedHashMap<K, V, RandomState> {
	fn from(value: HashMap<K, V>) -> Self {
		Self {
			data: value,
			aliases: HashMap::new(),
		}
	}
}

impl<K, V> AliasedHashMap<K, V, RandomState> {
	pub fn new() -> Self {
		Self {
			data: HashMap::new(),
			aliases: HashMap::new(),
		}
	}

	pub fn with_capacity(data_capacity: usize, alias_capacity: usize) -> Self {
		let data = HashMap::with_capacity_and_hasher(data_capacity, RandomState::new());
		let aliases = HashMap::with_capacity_and_hasher(alias_capacity, RandomState::new());
		Self { data, aliases }
	}

	pub fn with_data_capacity(capacity: usize) -> Self {
		let data = HashMap::with_capacity_and_hasher(capacity, RandomState::new());
		Self {
			data,
			aliases: HashMap::new(),
		}
	}

	pub fn with_alias_capacity(capacity: usize) -> Self {
		let aliases = HashMap::with_capacity_and_hasher(capacity, RandomState::new());
		Self {
			data: HashMap::new(),
			aliases,
		}
	}
}

impl<K, V, S: Clone> AliasedHashMap<K, V, S> {
	pub fn with_hasher(hash_builder: S) -> Self {
		Self {
			data: HashMap::with_hasher(hash_builder.clone()),
			aliases: HashMap::with_hasher(hash_builder),
		}
	}

	pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
		let data = HashMap::with_capacity_and_hasher(capacity, hasher.clone());
		let aliases = HashMap::with_capacity_and_hasher(capacity, hasher);
		Self { data, aliases }
	}

	pub fn capacity(&self) -> (usize, usize) {
		(self.data.capacity(), self.aliases.capacity())
	}

	pub fn data_capacity(&self) -> usize {
		self.data.capacity()
	}

	pub fn alias_capacity(&self) -> usize {
		self.aliases.capacity()
	}
}
