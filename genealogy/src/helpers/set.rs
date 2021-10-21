use std::collections::HashSet;

pub enum Set {}

impl Set {
	pub fn copy_of<Key>(set: &HashSet<Key>) -> HashSet<Key>
	where
		Key: Clone,
	{
		set.clone()
	}
}
