use crate::stream::Stream;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::hash::Hash;
use std::rc::Rc;

pub type JHashSet<Key> = Set<Key>;

#[derive(Debug, Clone)]
pub struct Set<Element> {
	set: Rc<RefCell<HashSet<Element>>>,
}

impl<Element> Set<Element> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			set: Default::default(),
		}
	}

	pub fn copy_of(set: Set<Element>) -> Set<Element>
	where
		Element: Clone,
	{
		set.set.as_ref().borrow().clone().into()
	}

	pub fn stream(self) -> Stream<Element>
	where
		Element: Clone + 'static,
	{
		Stream::of(self.set.as_ref().borrow().clone())
	}

	pub fn retain_all(&self, other: Self)
	where
		Element: Clone + Hash + Eq,
	{
		let mut set = self.set.borrow_mut();
		if other.set.as_ref().as_ptr() == self.set.as_ref().as_ptr() {
			// if both sets refer to the same underlying RefCell, it would
			// panic once "other" is borrowed, so return early. This is correct
			// since the intersection with itself is always the entire set, so
			// everything is retained.
			return;
		}

		let other_set = other.set.as_ref().borrow();
		let mut intersection = set.intersection(&other_set).cloned().collect::<HashSet<_>>();
		std::mem::swap(&mut intersection, &mut set);
	}

	pub fn contains(&self, other: &Element) -> bool
	where
		Element: Eq + Hash,
	{
		self.set.as_ref().borrow().contains(other)
	}

	pub fn size(&self) -> i32 {
		self.set.as_ref().borrow().len() as i32
	}
}

impl<Element> From<HashSet<Element>> for Set<Element> {
	fn from(hash_set: HashSet<Element>) -> Self {
		Self {
			set: Rc::new(RefCell::new(hash_set)),
		}
	}
}

impl<Element> IntoIterator for Set<Element>
where
	Element: Clone,
{
	type Item = Element;
	type IntoIter = <HashSet<Element> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.set.as_ref().borrow().clone().into_iter()
	}
}

impl<Element> PartialEq<HashSet<Element>> for Set<Element>
where
	HashSet<Element>: PartialEq,
{
	fn eq(&self, hash_set: &HashSet<Element>) -> bool {
		self.set.as_ref().borrow().eq(hash_set)
	}
}

impl<Element> PartialEq<Set<Element>> for HashSet<Element>
where
	HashSet<Element>: PartialEq,
{
	fn eq(&self, set: &Set<Element>) -> bool {
		set.set.as_ref().borrow().eq(self)
	}
}

impl<Element> Display for Set<Element>
where
	Element: Display,
{
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		let size = self.set.borrow().len();
		formatter.write_char('[')?;
		for (index, element) in self.set.as_ref().borrow().iter().enumerate() {
			element.fmt(formatter)?;
			if index < (size - 1) {
				formatter.write_str(", ")?;
			}
		}
		formatter.write_char(']')
	}
}
