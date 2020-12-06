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

#[cfg(test)]
mod test {
	use crate::helpers::exception::Exception::IllegalArgument;
	use crate::helpers::iterator::IteratorExtension;

	#[allow(clippy::unit_arg)]
	#[test]
	fn empty_stream_empty_optional() {
		let iterator = std::iter::empty::<()>();
		let option = iterator.equal().map(Result::unwrap).fold(None, |_, value| Some(value));
		assert!(option.is_none())
	}

	#[test]
	fn single_element_stream_optional_with_that_element() {
		let iterator = std::iter::once("element");
		let option = iterator.equal().map(Result::unwrap).fold(None, |_, value| Some(value));
		assert_eq!(Some("element"), option);
	}

	#[test]
	fn equal_element_stream_optional_with_that_element() {
		let iterator = vec!["element", "element", "element"].into_iter();
		let option = iterator.equal().map(Result::unwrap).fold(None, |_, value| Some(value));
		assert_eq!(Some("element"), option);
	}

	#[test]
	fn non_equal_element_stream_throws_exception() {
		let iterator = vec!["element", "other element"].into_iter();
		let result = iterator.equal().collect::<Result<Vec<_>, _>>();
		assert!(matches!(result, Err(IllegalArgument(_))))
	}
}
