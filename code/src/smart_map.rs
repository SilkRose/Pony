use std::collections::hash_map::{
	Drain, ExtractIf, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut,
};
use std::collections::{HashMap, HashSet, TryReserveError};
use std::hash::{BuildHasher, Hash, RandomState};
use std::sync::Arc;

pub struct SmartMap<K, V, S = RandomState> {
	pub set: HashSet<Arc<V>, S>,
	pub map: HashMap<K, Arc<V>, S>,
}

impl<K: Eq + Hash, V: Eq + Hash> Default for SmartMap<K, V, RandomState> {
	fn default() -> Self {
		Self {
			set: Default::default(),
			map: Default::default(),
		}
	}
}

impl<K: Eq + Hash, V: Eq + Hash> From<HashMap<K, V>> for SmartMap<K, V, RandomState> {
	fn from(map: HashMap<K, V>) -> Self {
		let mut res = Self::default();
		for (key, value) in map {
			res.insert(key, value);
		}
		res
	}
}

impl<K: Eq + Hash, V: Eq + Hash, S: Clone + BuildHasher> SmartMap<K, V, S> {
	pub fn with_hasher(hash_builder: S) -> Self {
		Self {
			set: HashSet::with_hasher(hash_builder.clone()),
			map: HashMap::with_hasher(hash_builder),
		}
	}

	pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
		let set = HashSet::with_capacity_and_hasher(capacity, hasher.clone());
		let map = HashMap::with_capacity_and_hasher(capacity, hasher);
		Self { set, map }
	}

	pub fn capacity(&self) -> (usize, usize) {
		(self.set.capacity(), self.map.capacity())
	}

	pub fn set_capacity(&self) -> usize {
		self.set.capacity()
	}

	pub fn map_capacity(&self) -> usize {
		self.map.capacity()
	}

	pub fn keys(&self) -> Keys<'_, K, Arc<V>> {
		self.map.keys()
	}

	pub fn into_keys(self) -> IntoKeys<K, Arc<V>> {
		self.map.into_keys()
	}

	pub fn values(&self) -> Values<'_, K, Arc<V>> {
		self.map.values()
	}

	pub fn values_mut(&mut self) -> ValuesMut<'_, K, Arc<V>> {
		self.map.values_mut()
	}

	pub fn into_values(self) -> IntoValues<K, Arc<V>> {
		self.map.into_values()
	}

	pub fn iter(&self) -> Iter<'_, K, Arc<V>> {
		self.map.iter()
	}

	pub fn iter_mut(&mut self) -> IterMut<'_, K, Arc<V>> {
		self.map.iter_mut()
	}

	pub fn len(&self) -> usize {
		self.set.len()
	}

	pub fn key_len(&self) -> usize {
		self.map.len()
	}

	pub fn is_empty(&self) -> bool {
		self.set.is_empty()
	}

	pub fn drain(&mut self) -> Drain<'_, K, Arc<V>> {
		self.map.drain()
	}

	pub fn extract_if<F>(&mut self, pred: F) -> ExtractIf<'_, K, Arc<V>, F>
	where
		F: FnMut(&K, &mut Arc<V>) -> bool,
	{
		self.map.extract_if(pred)
	}

	pub fn retain<F>(&mut self, f: F)
	where
		F: FnMut(&K, &mut Arc<V>) -> bool,
	{
		self.map.retain(f)
	}

	pub fn clear(&mut self) {
		self.set.clear();
		self.map.clear();
	}

	pub fn set_hasher(&self) -> &S {
		self.set.hasher()
	}

	pub fn map_hasher(&self) -> &S {
		self.map.hasher()
	}
}

impl<K: Eq + Hash, V: Eq + Hash, S: BuildHasher> SmartMap<K, V, S> {
	pub fn reserve(&mut self, additional: usize) {
		self.set.reserve(additional)
	}

	pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
		self.set.try_reserve(additional)
	}

	pub fn shrink_to_fit(&mut self) {
		self.set.shrink_to_fit();
	}

	pub fn shrink_to(&mut self, min_capacity: usize) {
		self.set.shrink_to(min_capacity);
	}

	pub fn insert(&mut self, k: K, v: V) -> Option<Arc<V>> {
		let value = Arc::new(v);
		self.set.insert(value.clone());
		self.map.insert(k, value)
	}

	pub fn insert_detected(&mut self, k: K, v: V) -> (bool, Option<Arc<V>>) {
		let value = Arc::new(v);
		let found = !self.set.insert(value.clone());
		(found, self.map.insert(k, value))
	}
}
