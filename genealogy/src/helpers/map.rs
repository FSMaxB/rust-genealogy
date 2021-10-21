use std::collections::HashMap;
use std::hash::Hash;

pub enum Map {}

impl Map {
	pub fn copy_of<Key, Value>(map: &HashMap<Key, Value>) -> HashMap<Key, Value>
	where
		Key: Clone,
		Value: Clone,
	{
		map.clone()
	}
}

pub trait MapExtension<Key, Value> {
	fn get_or_default(&self, key: &Key, default: Value) -> Value
	where
		Key: Eq + Hash,
		Value: Clone;
}

impl<Key, Value> MapExtension<Key, Value> for HashMap<Key, Value> {
	fn get_or_default(&self, key: &Key, default: Value) -> Value
	where
		Key: Eq + Hash,
		Value: Clone,
	{
		self.get(key).map(Clone::clone).unwrap_or(default)
	}
}

#[macro_export]
macro_rules! map_of {
	() => {
		::std::collections::HashMap::new()
	};
	($($key: expr, $value: expr), + $(,) ?) => {
		::literally::hmap!{
			$($key => $value),+
		}
	};
}

#[cfg(test)]
mod test {
	use literally::hmap;
	use std::collections::HashMap;

	#[test]
	fn map_of_none() {
		let map: HashMap<(), ()> = map_of!();
		assert!(map.is_empty())
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
