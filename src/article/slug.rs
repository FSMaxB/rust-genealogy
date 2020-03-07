use crate::exception::Exception;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Slug {
	pub value: String,
}

impl TryFrom<String> for Slug {
	type Error = Exception;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		if value.trim().is_empty() {
			Err(Exception::IllegalArgument(
				"Slugs can't have an empty value.".to_string(),
			))
		} else {
			Ok(Self { value })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn empty_text_exception() {
		let exception = Slug::try_from(String::from("")).unwrap_err();
		match exception {
			Exception::IllegalArgument(_) => (),
			_ => panic!("Not an IllegalArgument exception"),
		}
	}
}
