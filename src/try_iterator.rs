use std::iter::Iterator;

pub trait TryIterator: Iterator {
	type Ok;
	type Error;

	fn try_next(&mut self) -> Option<Result<Self::Ok, Self::Error>>;

	fn map_ok<Mapper, Output>(self, mapper: Mapper) -> TryFused<MapOk<Self, Mapper>>
	where
		Self: Sized,
		Mapper: FnMut(Self::Ok) -> Output;

	fn try_skip(self, amount: usize) -> TryFused<TrySkip<Self>>
	where
		Self: Sized;

	fn try_skip_while<Predicate>(self, predicate: Predicate) -> TryFused<TrySkipWhile<Self, Predicate>>
	where
		Self: Sized,
		Predicate: FnMut(&Self::Ok) -> bool;

	fn try_take_while<Predicate>(self, predicate: Predicate) -> TryFused<TryTakeWhile<Self, Predicate>>
	where
		Self: Sized,
		Predicate: FnMut(&Self::Ok) -> bool;

	fn try_fused(self) -> TryFused<Self>
	where
		Self: Sized;
}

impl<Iter, Ok, Error> TryIterator for Iter
where
	Iter: Iterator<Item = Result<Ok, Error>>,
{
	type Ok = Ok;
	type Error = Error;

	fn try_next(&mut self) -> Option<Result<Self::Ok, Self::Error>> {
		self.next()
	}

	fn map_ok<Mapper, Output>(self, mapper: Mapper) -> TryFused<MapOk<Self, Mapper>>
	where
		Mapper: FnMut(Self::Ok) -> Output,
	{
		MapOk {
			try_iterator: self,
			mapper,
		}
		.into()
	}

	fn try_skip(self, amount: usize) -> TryFused<TrySkip<Self>>
	where
		Self: Sized,
	{
		TrySkip {
			try_iterator: self,
			remaining: amount,
		}
		.into()
	}

	fn try_skip_while<Predicate>(self, predicate: Predicate) -> TryFused<TrySkipWhile<Self, Predicate>>
	where
		Predicate: FnMut(&Self::Ok) -> bool,
	{
		TrySkipWhile {
			try_iterator: self,
			finished_skipping: false,
			predicate,
		}
		.into()
	}

	fn try_take_while<Predicate>(self, predicate: Predicate) -> TryFused<TryTakeWhile<Self, Predicate>>
	where
		Self: Sized,
		Predicate: FnMut(&Self::Ok) -> bool,
	{
		TryTakeWhile {
			try_iterator: self,
			predicate,
			still_taking: true,
		}
		.into()
	}

	fn try_fused(self) -> TryFused<Self> {
		self.into()
	}
}

pub struct MapOk<TryIter, Mapper> {
	try_iterator: TryIter,
	mapper: Mapper,
}

impl<TryIter, Mapper, Output> Iterator for MapOk<TryIter, Mapper>
where
	TryIter: TryIterator,
	Mapper: FnMut(TryIter::Ok) -> Output,
{
	type Item = Result<Output, TryIter::Error>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.try_iterator.try_next() {
			None => None,
			Some(Ok(value)) => Some(Ok((self.mapper)(value))),
			Some(Err(error)) => Some(Err(error)),
		}
	}
}

pub struct TrySkipWhile<TryIter, Predicate> {
	try_iterator: TryIter,
	predicate: Predicate,
	finished_skipping: bool,
}

impl<TryIter, OkType, ErrorType, Predicate> Iterator for TrySkipWhile<TryIter, Predicate>
where
	TryIter: Iterator<Item = Result<OkType, ErrorType>>,
	Predicate: FnMut(&OkType) -> bool,
{
	type Item = TryIter::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.finished_skipping {
			self.try_iterator.next()
		} else {
			let predicate = &mut self.predicate;
			let item = self.try_iterator.find(|item| match item {
				Ok(value) => !(predicate)(value),
				Err(_) => true,
			});
			self.finished_skipping = true;
			item
		}
	}
}

pub struct TryFused<TryIter> {
	try_iterator: TryIter,
	finished: bool,
}

impl<TryIter> From<TryIter> for TryFused<TryIter> {
	fn from(try_iterator: TryIter) -> Self {
		Self {
			try_iterator,
			finished: false,
		}
	}
}

impl<TryIter, OkType, ErrorType> Iterator for TryFused<TryIter>
where
	TryIter: Iterator<Item = Result<OkType, ErrorType>>,
{
	type Item = TryIter::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.finished {
			None
		} else {
			match self.try_iterator.next() {
				None => {
					self.finished = true;
					None
				}
				Some(result) => {
					self.finished = result.is_err();
					Some(result)
				}
			}
		}
	}
}

pub struct TrySkip<TryIter> {
	try_iterator: TryIter,
	remaining: usize,
}

impl<TryIter, OkType, ErrorType> Iterator for TrySkip<TryIter>
where
	TryIter: Iterator<Item = Result<OkType, ErrorType>>,
{
	type Item = TryIter::Item;

	fn next(&mut self) -> Option<Self::Item> {
		while self.remaining > 0 {
			match self.try_iterator.next() {
				None => return None,
				Some(result) => {
					if result.is_err() {
						return Some(result);
					}
				}
			};
			self.remaining -= 1;
		}

		self.try_iterator.next()
	}
}

pub struct TryTakeWhile<TryIter, Predicate> {
	try_iterator: TryIter,
	predicate: Predicate,
	still_taking: bool,
}

impl<TryIter, OkType, ErrorType, Predicate> Iterator for TryTakeWhile<TryIter, Predicate>
where
	TryIter: Iterator<Item = Result<OkType, ErrorType>>,
	Predicate: FnMut(&OkType) -> bool,
{
	type Item = TryIter::Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.still_taking {
			let predicate = &mut self.predicate;
			match self.try_iterator.next() {
				None => None,
				Some(Ok(value)) => {
					if predicate(&value) {
						Some(Ok(value))
					} else {
						self.still_taking = false;
						None
					}
				}
				Some(Err(error)) => Some(Err(error)),
			}
		} else {
			None
		}
	}
}
