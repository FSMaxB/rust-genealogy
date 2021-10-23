use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Map<Key, Value> {
	map: Rc<HashMap<Key, Value>>,
}

impl<Key, Value> Map<Key, Value> {
	pub fn copy_of(map: Map<Key, Value>) -> Map<Key, Value>
	where
		Key: Clone,
		Value: Clone,
	{
		map.map.as_ref().clone().into()
	}

	pub fn get(&self, key: Key) -> Option<Value>
	where
		Key: Hash + Eq,
		Value: Clone,
	{
		self.map.get(&key).cloned()
	}

	pub fn get_or_default(&self, key: Key, default: Value) -> Value
	where
		Key: Eq + Hash,
		Value: Clone,
	{
		self.map.get(&key).map(Clone::clone).unwrap_or(default)
	}
}

impl<Key, Value> From<HashMap<Key, Value>> for Map<Key, Value> {
	fn from(hash_map: HashMap<Key, Value>) -> Self {
		Self { map: Rc::new(hash_map) }
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

#[cfg(test)]
mod test {
	use super::*;
	use literally::hmap;
	use std::collections::HashMap;

	#[test]
	fn map_of_none() {
		let map: Map<(), ()> = map_of!();
		assert!(map.map.is_empty())
	}

	#[test]
	fn map_of_one() {
		let expected: HashMap<&'static str, &'static str> = hmap! {"hello" => "world"};
		let actual = map_of!("hello", "world");

		assert_eq!(&expected, actual.map.as_ref());
	}

	#[test]
	fn map_of_two() {
		let expected: HashMap<&'static str, &'static str> = hmap! {"hello" => "hello", "world" => "world"};
		let actual = map_of!("hello", "hello", "world", "world");
		assert_eq!(&expected, actual.map.as_ref());
	}
}
