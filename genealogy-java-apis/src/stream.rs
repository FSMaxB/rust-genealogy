use crate::collector::Collector;
use crate::comparator::Comparator;
use crate::exception::Exception;
use crate::exception::Exception::IllegalArgumentException;
use crate::integer::Integer;
use crate::list::List;
use crate::throw;
use std::convert::identity;
use std::iter::once;

pub struct Stream<Item> {
	iterator: Box<dyn Iterator<Item = Result<Item, Exception>> + 'static>,
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

	pub fn flat_map<NewItem>(self, mut mapper: impl FnMut(Item) -> Stream<NewItem> + 'static) -> Stream<NewItem>
	where
		NewItem: 'static,
	{
		self.iterator
			.flat_map(move |result| {
				result
					.map(|item| mapper(item).iterator)
					.unwrap_or_else(|exception| Box::new(once(Err(exception))))
			})
			.into()
	}

	pub fn filter(self, mut predicate: impl FnMut(&Item) -> bool + 'static) -> Self {
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

	pub fn of<Iterable>(iterable: Iterable) -> Stream<Iterable::Item>
	where
		Iterable: IntoIterator<Item = Item> + 'static,
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

	pub fn sorted(self, comparator: Comparator<Item>) -> Result<Self, Exception> {
		let mut items = self.iterator.collect::<Result<Vec<_>, _>>()?;
		items.sort_by(comparator.compare);
		Ok(Stream::of(items))
	}

	pub fn into_iterator(self) -> Box<dyn Iterator<Item = Result<Item, Exception>> + 'static> {
		self.iterator
	}

	pub fn for_each(self, mut action: impl FnMut(Item) -> Result<(), Exception> + 'static) -> Result<(), Exception> {
		self.iterator.fold(Ok(()), move |result, item| match result {
			Ok(_) => item.map(|item| action(item)).and_then(identity),
			exception => exception,
		})
	}

	pub fn count(self) -> Result<i64, Exception> {
		self.iterator.fold(Ok(0), |result, item| match result {
			Ok(count) => item.map(|_| count + 1),
			exception => exception,
		})
	}
}

impl Stream<i32> {
	pub fn boxed(self) -> Stream<Integer> {
		self.map(|integer| Ok(integer.into()))
	}
}

impl<Iterable, Item, Error> From<Iterable> for Stream<Item>
where
	Iterable: IntoIterator<Item = Result<Item, Error>> + 'static,
	Error: Into<Exception> + 'static,
{
	fn from(iterable: Iterable) -> Self {
		Self {
			iterator: Box::new(iterable.into_iter().map(|result| result.map_err(Into::into))),
		}
	}
}

pub trait Streamable {
	type Item;

	fn into_stream(self) -> Stream<Self::Item>;
}

impl<Item> Streamable for Stream<Item> {
	type Item = Item;

	fn into_stream(self) -> Stream<Self::Item> {
		self
	}
}

impl<Iterable> Streamable for Iterable
where
	Iterable: IntoIterator,
	Iterable::Item: 'static,
	Iterable::IntoIter: 'static,
{
	type Item = Iterable::Item;

	fn into_stream(self) -> Stream<Self::Item> {
		self.into_iter().map(Result::<_, Exception>::Ok).into()
	}
}
