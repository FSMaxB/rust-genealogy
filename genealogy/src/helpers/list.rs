use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IndexOutOfBoundsException;
use crate::helpers::stream::Stream;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

pub type ArrayList<Element> = List<Element>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct List<Element> {
	vector: Rc<RefCell<Vec<Element>>>,
}

impl<Element> List<Element> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			vector: Default::default(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.vector.as_ref().borrow().is_empty()
	}

	pub fn get(&self, index: usize) -> Result<Element, Exception>
	where
		Element: Clone,
	{
		self.vector
			.as_ref()
			.borrow()
			.get(index)
			.cloned()
			.ok_or(IndexOutOfBoundsException(index))
	}

	pub fn copy_of(list: impl IntoIterator<Item = Element>) -> Self
	where
		Element: Clone,
	{
		list.into_iter().collect::<Vec<_>>().into()
	}

	pub fn of(iterable: impl IntoIterator<Item = Element>) -> Self {
		iterable.into_iter().collect()
	}

	pub fn stream(&self) -> Stream<Element>
	where
		Element: Clone + 'static,
	{
		Stream::of(self.vector.as_ref().borrow().clone())
	}

	pub fn add(&mut self, element: Element) -> bool
	where
		Element: PartialEq,
	{
		let mut vector = self.vector.borrow_mut();
		if !vector.contains(&element) {
			vector.push(element);
			true
		} else {
			false
		}
	}

	pub fn length(&self) -> usize {
		self.vector.as_ref().borrow().len()
	}
}

impl<Element> Hash for List<Element>
where
	Vec<Element>: Hash,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.vector.as_ref().borrow().hash(state)
	}
}

impl<Element> Serialize for List<Element>
where
	Element: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.vector.as_ref().serialize(serializer)
	}
}

impl<'de, Element> Deserialize<'de> for List<Element>
where
	Element: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Self {
			vector: Rc::new(Vec::deserialize(deserializer)?.into()),
		})
	}
}

impl<Element> FromIterator<Element> for List<Element> {
	fn from_iter<Iterable: IntoIterator<Item = Element>>(iterable: Iterable) -> Self {
		Self {
			vector: Rc::new(Vec::from_iter(iterable).into()),
		}
	}
}

impl<Element> PartialEq<Vec<Element>> for List<Element>
where
	Element: PartialEq,
{
	fn eq(&self, other: &Vec<Element>) -> bool {
		self.vector.as_ref().borrow().deref() == other
	}
}

impl<Element> PartialEq<List<Element>> for Vec<Element>
where
	Element: PartialEq,
{
	fn eq(&self, other: &List<Element>) -> bool {
		other.vector.as_ref().borrow().deref() == self
	}
}

impl<Element> IntoIterator for List<Element>
where
	Element: Clone,
{
	type Item = Element;
	type IntoIter = <Vec<Element> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.vector.as_ref().borrow().clone().into_iter()
	}
}

impl<Element> From<Vec<Element>> for List<Element> {
	fn from(vector: Vec<Element>) -> Self {
		Self {
			vector: Rc::new(vector.into()),
		}
	}
}
