use crate::helpers::exception::Exception;

pub struct Stream<Item> {
	iterator: Box<dyn Iterator<Item = Result<Item, Exception>>>,
}

impl<Item> Stream<Item> {
	/// Emulate Stream.toArray()
	pub fn to_vector(self, generator: impl FnOnce(usize) -> Vec<Item>) -> Result<Vec<Item>, Exception> {
		let Self { mut iterator } = self;

		let (size_estimate, _) = iterator.size_hint();
		let mut vector = generator(size_estimate);

		for item in iterator {
			let item = item?;
			vector.push(item);
		}

		Ok(vector)
	}
}

impl<IteratorType, Item> From<IteratorType> for Stream<Item>
where
	IteratorType: Iterator<Item = Result<Item, Exception>> + 'static,
{
	fn from(iterator: IteratorType) -> Self {
		Self {
			iterator: Box::new(iterator),
		}
	}
}
