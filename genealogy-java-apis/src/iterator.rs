pub trait ResultIteratorExtension<OkIterator, Error> {
	fn into_result_iterator(self) -> ResultIterator<OkIterator, Error>;
}

impl<IntoOkIterator, OkIterator, OkType, Error> ResultIteratorExtension<OkIterator, Error>
	for Result<IntoOkIterator, Error>
where
	IntoOkIterator: IntoIterator<Item = OkType, IntoIter = OkIterator>,
	OkIterator: Iterator<Item = OkType>,
{
	/// Convert a `Result<Iterator<Item = Element>, Error>` to `Iterator<Item = Result<Element, Error>>`
	fn into_result_iterator(self) -> ResultIterator<OkIterator, Error> {
		match self {
			Ok(iterator) => ResultIterator::Ok(iterator.into_iter()),
			Err(error) => ResultIterator::Err(error),
		}
	}
}

pub enum ResultIterator<OkIterator, Error> {
	Ok(OkIterator),
	Err(Error),
	Finished,
}

impl<OkIterator, Error> Default for ResultIterator<OkIterator, Error> {
	fn default() -> Self {
		Self::Finished
	}
}

impl<OkIterator, OkType, Error> Iterator for ResultIterator<OkIterator, Error>
where
	OkIterator: Iterator<Item = OkType>,
{
	type Item = Result<OkType, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		let mut iterator = match std::mem::take(self) {
			Self::Ok(iterator) => iterator,
			Self::Err(error) => {
				return Some(Err(error));
			}
			Self::Finished => {
				return None;
			}
		};

		let next = iterator.next()?;

		*self = Self::Ok(iterator);
		Some(Ok(next))
	}
}
