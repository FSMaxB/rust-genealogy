use crate::helpers::collector::Collector;
use crate::helpers::exception::Exception;
use resiter::{Filter, FlatMap, Map};
use std::convert::identity;

pub struct Stream<Item> {
	iterator: Box<dyn Iterator<Item = Result<Item, Exception>>>,
}

impl<Item> Stream<Item>
where
	Item: 'static,
{
	pub fn map<NewItem>(self, mut mapper: impl FnMut(Item) -> Result<NewItem, Exception> + 'static) -> Stream<NewItem>
	where
		NewItem: 'static,
	{
		self.iterator
			.map(move |result| result.map(|item| (&mut mapper)(item)).and_then(identity))
			.into()
	}

	pub fn flat_map<NewItem>(self, mut mapper: impl FnMut(Item) -> Option<Stream<NewItem>> + 'static) -> Stream<NewItem>
	where
		NewItem: 'static,
	{
		self.iterator
			.flat_map_ok(move |item| mapper(item).into_iter().flat_map(|stream| stream.iterator))
			.map(|result| result.and_then(identity))
			.into()
	}

	pub fn filter(self, predicate: impl FnMut(&Item) -> bool + 'static) -> Self {
		self.iterator.filter_ok(predicate).into()
	}

	pub fn collect<Accumulated, Reduced>(
		self,
		collector: Collector<Item, Accumulated, Reduced>,
	) -> Result<Reduced, Exception> {
		let mut accumulated = (collector.supplier)()?;
		for item in self.iterator {
			let item = item?;
			(collector.accumulator)(&mut accumulated, item)?;
		}
		(collector.finisher)(accumulated)
	}

	pub fn of<Iterable>(iterable: Iterable) -> Stream<Iterable::Item>
	where
		Iterable: IntoIterator<Item = Item> + 'static,
	{
		iterable.into_iter().map(Result::<_, Exception>::Ok).into()
	}

	pub fn limit(self, limit: usize) -> Self {
		self.iterator.take(limit).into()
	}

	pub fn to_list(self) -> Result<Vec<Item>, Exception> {
		self.iterator.collect()
	}

	pub fn drop_while(self, predicate: impl Fn(&Item) -> bool + 'static) -> Self {
		self.iterator
			.skip_while(move |result| result.as_ref().map(|item| (&predicate)(item)).unwrap_or(false))
			.into()
	}

	pub fn take_while(self, predicate: impl Fn(&Item) -> bool + 'static) -> Self {
		self.iterator
			.take_while(move |result| result.as_ref().map(|item| (&predicate)(item)).unwrap_or(false))
			.into()
	}

	pub fn skip(self, amount: usize) -> Self {
		self.iterator.skip(amount).into()
	}
}

impl<Iter, Item, Error> From<Iter> for Stream<Item>
where
	Iter: Iterator<Item = Result<Item, Error>> + 'static,
	Error: Into<Exception> + 'static,
{
	fn from(iterator: Iter) -> Self {
		Self {
			iterator: Box::new(iterator.map_err(Into::into)),
		}
	}
}

impl<Item> From<Stream<Item>> for Box<dyn Iterator<Item = Result<Item, Exception>>> {
	fn from(stream: Stream<Item>) -> Self {
		stream.iterator
	}
}

impl<Item> IntoIterator for Stream<Item> {
	type Item = Result<Item, Exception>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

	fn into_iter(self) -> Self::IntoIter {
		self.iterator
	}
}

pub trait StreamExtensions {
	type Item;

	fn stream(self) -> Stream<Self::Item>;
}

impl<Item> StreamExtensions for Vec<Item>
where
	Item: 'static,
{
	type Item = Item;

	fn stream(self) -> Stream<Self::Item> {
		Stream::of(self)
	}
}

#[macro_export]
macro_rules! stream_of {
	() => {
		{
			use ::std::result::Result;
			use ::std::convert::From;
			use crate::helpers::stream::Stream;
			use ::std::iter::empty;
			use crate::helpers::exception::Exception;
			<Stream<_> as From<_>>::from(empty::<Result<_,Exception>>())
		}
	};
	($($element: expr), + $(,) ?) => {
		{
			use ::std::result::Result;
			use ::std::convert::From;
			use crate::helpers::stream::Stream;
			use ::std::iter::IntoIterator;
			use ::std::iter::Iterator;
			use crate::helpers::exception::Exception;
			<Stream<_> as From<_>>::from(::std::vec![$($element),+].into_iter().map(Result::Ok::<_, Exception>))
		}
	};
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn stream_of_none() {
		let mut stream: Stream<String> = stream_of!();
		assert!(stream.iterator.next().is_none());
	}

	#[test]
	fn stream_of_one() {
		let mut stream = stream_of!("hello");
		assert_eq!("hello", stream.iterator.next().unwrap().unwrap());
		assert!(stream.iterator.next().is_none());
	}

	#[test]
	fn stream_of_two() {
		let mut stream = stream_of!("hello", "world");
		assert_eq!("hello", stream.iterator.next().unwrap().unwrap());
		assert_eq!("world", stream.iterator.next().unwrap().unwrap());
		assert!(stream.iterator.next().is_none());
	}
}
