pub enum List {}

impl List {
	pub fn copy_of<Element>(list: &[Element]) -> Vec<Element>
	where
		Element: Clone,
	{
		list.iter().cloned().collect()
	}
}

#[macro_export]
macro_rules! list_of {
	() => {
		::std::vec::Vec::new()
	};
	($($element: expr), + $(,) ?) => {
		::std::vec![$($element),+]
	};
}

#[cfg(test)]
mod test {
	#[test]
	fn list_of_none() {
		let list: Vec<()> = list_of!();
		assert!(list.is_empty())
	}

	#[test]
	fn list_of_one() {
		assert_eq!(vec!["hello"], list_of!("hello"));
	}

	#[test]
	fn list_of_two() {
		assert_eq!(vec!["hello", "world"], list_of!("hello", "world"));
	}
}
