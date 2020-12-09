use crate::helpers::iterator::equal::Equal;
use crate::helpers::iterator::split_pair::{SplitPairLeft, SplitPairRight};
use std::fmt::Debug;
use std::iter::Iterator;

pub mod equal;
pub mod result_iterator;
pub mod split_pair;

pub trait IteratorExtension: Sized {
	// NOTE: This roughly works as an equivalent to the `tee` collector in the original code base
	fn split_pair<ElementLeft, ElementRight>(
		self,
	) -> (
		SplitPairLeft<ElementLeft, ElementRight, Self>,
		SplitPairRight<ElementLeft, ElementRight, Self>,
	)
	where
		Self: Iterator<Item = (ElementLeft, ElementRight)>;

	fn equal<Element>(self) -> Equal<Element, Self>
	where
		Self: Iterator<Item = Element>,
		Element: Clone + Debug + Eq;
}

impl<T> IteratorExtension for T {
	fn split_pair<ElementLeft, ElementRight>(
		self,
	) -> (
		SplitPairLeft<ElementLeft, ElementRight, Self>,
		SplitPairRight<ElementLeft, ElementRight, Self>,
	)
	where
		Self: Iterator<Item = (ElementLeft, ElementRight)>,
	{
		split_pair::split_pair(self)
	}

	fn equal<Element>(self) -> Equal<Element, Self>
	where
		Self: Iterator<Item = Element>,
		Element: Clone + Debug + Eq,
	{
		equal::equal(self)
	}
}
