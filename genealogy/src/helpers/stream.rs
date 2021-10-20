use crate::helpers::collector::Collector;
use crate::helpers::exception::Exception;
use resiter::{FlatMap, Map};
use std::convert::identity;

pub struct Stream<Item> {
	iterator: Box<dyn Iterator<Item = Result<Item, Exception>>>,
}

impl<Item> Stream<Item>
where
	Item: 'static,
{
	pub fn flat_map<NewItem>(self, mut mapper: impl FnMut(Item) -> Option<Stream<NewItem>> + 'static) -> Stream<NewItem>
	where
		NewItem: 'static,
	{
		self.iterator
			.flat_map_ok(move |item| mapper(item).into_iter().flat_map(|stream| stream.iterator))
			.map(|result| result.and_then(identity))
			.into()
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
}

impl<Iterable, Item, Error> From<Iterable> for Stream<Item>
where
	Iterable: IntoIterator<Item = Result<Item, Error>> + 'static,
	Error: Into<Exception> + 'static,
{
	fn from(iterable: Iterable) -> Self {
		Self {
			iterator: Box::new(iterable.into_iter().map_err(Into::into)),
		}
	}
}

impl<Item> From<Stream<Item>> for Box<dyn Iterator<Item = Result<Item, Exception>>> {
	fn from(stream: Stream<Item>) -> Self {
		stream.iterator
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
