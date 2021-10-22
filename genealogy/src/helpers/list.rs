use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IndexOutOfBoundsException;
use crate::helpers::stream::{Stream, StreamExtensions};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct List<Element> {
	vector: Rc<Vec<Element>>,
}

impl<Element> List<Element> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self {
			vector: Default::default(),
		}
	}

	pub fn get(&self, index: usize) -> Result<&Element, Exception> {
		self.vector.get(index).ok_or(IndexOutOfBoundsException(index))
	}

	pub fn copy_of(list: &[Element]) -> Self
	where
		Element: Clone,
	{
		list.to_vec().into()
	}

	pub fn of(iterable: impl IntoIterator<Item = Element>) -> Self {
		iterable.into_iter().collect()
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
			vector: Rc::new(Vec::deserialize(deserializer)?),
		})
	}
}

impl<Element> Deref for List<Element> {
	type Target = [Element];

	fn deref(&self) -> &Self::Target {
		self.vector.as_ref()
	}
}

impl<Element> FromIterator<Element> for List<Element> {
	fn from_iter<Iterable: IntoIterator<Item = Element>>(iterable: Iterable) -> Self {
		Self {
			vector: Rc::new(Vec::from_iter(iterable)),
		}
	}
}

impl<Element> PartialEq<Vec<Element>> for List<Element>
where
	Element: PartialEq,
{
	fn eq(&self, other: &Vec<Element>) -> bool {
		self.vector.as_ref() == other
	}
}

impl<Element> PartialEq<List<Element>> for Vec<Element>
where
	Element: PartialEq,
{
	fn eq(&self, other: &List<Element>) -> bool {
		other.vector.as_ref() == self
	}
}

impl<Element> StreamExtensions<'static> for List<Element>
where
	Element: Clone + 'static,
{
	type Item = Element;

	fn stream(self) -> Stream<'static, Self::Item> {
		Stream::of(self.vector.as_ref().clone().into_iter())
	}
}

impl<Element> IntoIterator for List<Element>
where
	Element: Clone,
{
	type Item = Element;
	type IntoIter = <Vec<Element> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.vector.as_ref().clone().into_iter()
	}
}

impl<Element> From<Vec<Element>> for List<Element> {
	fn from(vector: Vec<Element>) -> Self {
		Self {
			vector: Rc::new(vector),
		}
	}
}
