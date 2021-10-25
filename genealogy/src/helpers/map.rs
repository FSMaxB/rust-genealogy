use crate::helpers::collection::Collection;
use crate::helpers::set::Set;
use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type JHashMap<Key, Value> = Map<Key, Value>;

#[derive(Clone, Debug)]
pub struct Map<Key, Value> {
	map: Rc<RefCell<HashMap<Key, Value>>>,
}

impl<Key, Value> Map<Key, Value> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			map: Default::default(),
		}
	}

	pub fn copy_of(map: Map<Key, Value>) -> Map<Key, Value>
	where
		Key: Clone,
		Value: Clone,
	{
		map.map.as_ref().borrow().clone().into()
	}

	pub fn get(&self, key: Key) -> Option<Value>
	where
		Key: Hash + Eq,
		Value: Clone,
	{
		self.map.as_ref().borrow().get(&key).cloned()
	}

	pub fn get_or_default(&self, key: Key, default: Value) -> Value
	where
		Key: Eq + Hash,
		Value: Clone,
	{
		self.map
			.as_ref()
			.borrow()
			.get(&key)
			.map(Clone::clone)
			.unwrap_or(default)
	}

	pub fn entry_set(self) -> Set<Entry<Key, Value>>
	where
		Key: Clone + Eq + Hash,
		Value: Clone,
	{
		self.map
			.as_ref()
			.borrow()
			.iter()
			.map(|(key, value)| Entry {
				key: key.clone(),
				value: value.clone(),
			})
			.collect::<HashSet<_>>()
			.into()
	}

	pub fn compute_if_absent(
		&mut self,
		key: Key,
		mapping_function: impl FnOnce(Key) -> Value + 'static,
	) -> RefMut<Value>
	where
		Key: Clone + Eq + Hash,
	{
		RefMut::map(self.map.borrow_mut(), move |hash_map| {
			hash_map
				.entry(key)
				.or_insert_with_key(|key| mapping_function(key.clone()))
		})
	}

	pub fn values(self) -> Collection<Value>
	where
		Value: Clone,
	{
		self.map.as_ref().borrow().values().cloned().collect::<Vec<_>>().into()
	}
}

#[derive(Clone)]
pub struct Entry<Key, Value> {
	key: Key,
	value: Value,
}

impl<Key, Value> Entry<Key, Value> {
	pub fn get_key(&self) -> Key
	where
		Key: Clone,
	{
		self.key.clone()
	}

	pub fn get_value(&self) -> Value
	where
		Value: Clone,
	{
		self.value.clone()
	}
}

impl<Key, Value> PartialEq for Entry<Key, Value>
where
	Key: Eq,
{
	fn eq(&self, other: &Self) -> bool {
		self.key.eq(&other.key)
	}
}

impl<Key, Value> Eq for Entry<Key, Value> where Key: Eq {}

impl<Key, Value> Hash for Entry<Key, Value>
where
	Key: Hash,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.key.hash(state)
	}
}

impl<Key, Value> From<HashMap<Key, Value>> for Map<Key, Value> {
	fn from(hash_map: HashMap<Key, Value>) -> Self {
		Self {
			map: Rc::new(hash_map.into()),
		}
	}
}

#[macro_export]
macro_rules! map_of {
	() => {
		crate::helpers::map::Map::from(::std::collections::HashMap::new())
	};
	($($key: expr, $value: expr), + $(,) ?) => {
		crate::helpers::map::Map::from(::literally::hmap!{
			$($key => $value),+
		})
	};
}

impl<Key, Value> PartialEq<HashMap<Key, Value>> for Map<Key, Value>
where
	HashMap<Key, Value>: PartialEq,
{
	fn eq(&self, hash_map: &HashMap<Key, Value>) -> bool {
		self.map.as_ref().borrow().eq(hash_map)
	}
}

impl<Key, Value> PartialEq<Map<Key, Value>> for HashMap<Key, Value>
where
	HashMap<Key, Value>: PartialEq,
{
	fn eq(&self, map: &Map<Key, Value>) -> bool {
		map.map.as_ref().borrow().eq(self)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use literally::hmap;
	use std::collections::HashMap;

	#[test]
	fn map_of_none() {
		let map: Map<(), ()> = map_of!();
		assert!(map.map.as_ref().borrow().is_empty())
	}

	#[test]
	fn map_of_one() {
		let expected: HashMap<&'static str, &'static str> = hmap! {"hello" => "world"};
		let actual = map_of!("hello", "world");

		assert_eq!(expected, actual);
	}

	#[test]
	fn map_of_two() {
		let expected: HashMap<&'static str, &'static str> = hmap! {"hello" => "hello", "world" => "world"};
		let actual = map_of!("hello", "hello", "world", "world");
		assert_eq!(expected, actual);
	}
}
