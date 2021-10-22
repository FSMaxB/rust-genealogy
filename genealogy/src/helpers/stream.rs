use crate::helpers::collector::Collector;
use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use resiter::{Filter, FlatMap, Map};
use std::convert::identity;

pub struct Stream<'a, Item> {
	iterator: Box<dyn Iterator<Item = Result<Item, Exception>> + 'a>,
}

impl<'a, Item> Stream<'a, Item>
where
	Item: 'a,
{
	pub fn map<NewItem>(self, mut mapper: impl FnMut(Item) -> Result<NewItem, Exception> + 'a) -> Stream<'a, NewItem>
	where
		NewItem: 'a,
	{
		self.iterator
			.map(move |result| result.map(|item| (&mut mapper)(item)).and_then(identity))
			.into()
	}

	pub fn flat_map<NewItem>(
		self,
		mut mapper: impl FnMut(Item) -> Option<Stream<'a, NewItem>> + 'a,
	) -> Stream<'a, NewItem>
	where
		NewItem: 'a,
	{
		self.iterator
			.flat_map_ok(move |item| mapper(item).into_iter().flat_map(|stream| stream.iterator))
			.map(|result| result.and_then(identity))
			.into()
	}

	pub fn filter(self, predicate: impl FnMut(&Item) -> bool + 'a) -> Self {
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

	pub fn of<Iterable>(iterable: Iterable) -> Stream<'a, Iterable::Item>
	where
		Iterable: IntoIterator<Item = Item> + 'a,
	{
		iterable.into_iter().map(Result::<_, Exception>::Ok).into()
	}

	pub fn limit(self, limit: usize) -> Self {
		self.iterator.take(limit).into()
	}

	pub fn to_list(self) -> Result<List<Item>, Exception> {
		self.iterator.collect()
	}

	pub fn drop_while(self, predicate: impl Fn(&Item) -> bool + 'a) -> Self {
		self.iterator
			.skip_while(move |result| result.as_ref().map(|item| (&predicate)(item)).unwrap_or(false))
			.into()
	}

	pub fn take_while(self, predicate: impl Fn(&Item) -> bool + 'a) -> Self {
		self.iterator
			.take_while(move |result| result.as_ref().map(|item| (&predicate)(item)).unwrap_or(false))
			.into()
	}

	pub fn skip(self, amount: usize) -> Self {
		self.iterator.skip(amount).into()
	}
}

impl<'a, Iter, Item, Error> From<Iter> for Stream<'a, Item>
where
	Iter: Iterator<Item = Result<Item, Error>> + 'a,
	Error: Into<Exception> + 'static,
{
	fn from(iterator: Iter) -> Self {
		Self {
			iterator: Box::new(iterator.map_err(Into::into)),
		}
	}
}

impl<'a, Item> From<Stream<'a, Item>> for Box<dyn Iterator<Item = Result<Item, Exception>> + 'a> {
	fn from(stream: Stream<'a, Item>) -> Self {
		stream.iterator
	}
}

impl<'a, Item> IntoIterator for Stream<'a, Item> {
	type Item = Result<Item, Exception>;
	type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

	fn into_iter(self) -> Self::IntoIter {
		self.iterator
	}
}

pub trait StreamExtensions<'a> {
	type Item;

	fn stream(self) -> Stream<'a, Self::Item>;
}

impl<'a, Item> StreamExtensions<'a> for &'a [Item]
where
	Item: 'a,
{
	type Item = &'a Item;

	fn stream(self) -> Stream<'a, Self::Item> {
		Stream::of(self.iter())
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
