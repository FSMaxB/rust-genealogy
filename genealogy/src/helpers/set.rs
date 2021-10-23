use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Set<Key> {
	set: Rc<HashSet<Key>>,
}

impl<Key> Set<Key> {
	pub fn copy_of(set: Set<Key>) -> Set<Key>
	where
		Key: Clone,
	{
		set.set.as_ref().clone().into()
	}
}

impl<Key> From<HashSet<Key>> for Set<Key> {
	fn from(hash_set: HashSet<Key>) -> Self {
		Self { set: Rc::new(hash_set) }
	}
}

impl<Key> AsRef<HashSet<Key>> for Set<Key> {
	fn as_ref(&self) -> &HashSet<Key> {
		self.set.as_ref()
	}
}

impl<Key> IntoIterator for Set<Key>
where
	Key: Clone,
{
	type Item = Key;
	type IntoIter = <HashSet<Key> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.set.as_ref().clone().into_iter()
	}
}
