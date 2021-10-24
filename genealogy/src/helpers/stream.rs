use crate::helpers::collector::Collector;
use crate::helpers::comparator::Comparator;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::list::List;
use crate::throw;
use resiter::FlatMap;
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

	pub fn flat_map<NewItem>(self, mut mapper: impl FnMut(Item) -> Stream<'a, NewItem> + 'a) -> Stream<'a, NewItem>
	where
		NewItem: 'a,
	{
		self.iterator
			.flat_map_ok(move |item| mapper(item).iterator)
			.map(|result| result.and_then(identity))
			.into()
	}

	pub fn filter(self, mut predicate: impl FnMut(&Item) -> bool + 'a) -> Self {
		self.iterator
			.filter(move |result| result.as_ref().ok().map(|item| predicate(item)).unwrap_or(true))
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

	pub fn of<Iterable>(iterable: Iterable) -> Stream<'a, Iterable::Item>
	where
		Iterable: IntoIterator<Item = Item> + 'a,
	{
		iterable.into_iter().map(Result::<_, Exception>::Ok).into()
	}

	pub fn limit(self, limit: i32) -> Result<Self, Exception> {
		if limit < 0 {
			throw!(IllegalArgumentException(
				format!("Limit must be non-negative but was {}", limit).into()
			));
		}
		Ok(self.iterator.take(limit as usize).into())
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

	pub fn sorted(self, comparator: Comparator<Item>) -> Result<Self, Exception> {
		let mut items = self.iterator.collect::<Result<Vec<_>, _>>()?;
		items.sort_by(comparator.compare);
		Ok(Stream::of(items))
	}
}

impl<'a, Iter, Item, Error> From<Iter> for Stream<'a, Item>
where
	Iter: Iterator<Item = Result<Item, Error>> + 'a,
	Error: Into<Exception> + 'static,
{
	fn from(iterator: Iter) -> Self {
		Self {
			iterator: Box::new(iterator.map(|result| result.map_err(Into::into))),
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
