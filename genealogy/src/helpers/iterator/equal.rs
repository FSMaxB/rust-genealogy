use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use std::fmt::Debug;

pub fn equal<Element, Iter>(iterator: Iter) -> Equal<Element, Iter>
where
	Iter: Iterator<Item = Element>,
{
	Equal {
		iterator,
		previous: None,
	}
}

pub struct Equal<Element, Iter> {
	iterator: Iter,
	previous: Option<Element>,
}

impl<Element, Iter> Iterator for Equal<Element, Iter>
where
	Iter: Iterator<Item = Element>,
	Element: Clone + Debug + Eq,
{
	type Item = Result<Element, Exception>;

	fn next(&mut self) -> Option<Self::Item> {
		let element = self.iterator.next()?;

		let previous = self.previous.get_or_insert_with(|| element.clone());
		if &element == previous {
			Some(Ok(element))
		} else {
			Some(Err(IllegalArgument(format!(
				"Unequal elements in stream: {:?} vs {:?}",
				previous, element
			))))
		}
	}
}
