use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug {
	pub value: String,
}

impl Slug {
	pub fn from_value(value: String) -> Result<Slug, Exception> {
		if value.trim().is_empty() {
			Err(IllegalArgument("Slugs can't have an empty value.".to_string()))
		} else {
			Ok(Slug { value })
		}
	}

	// NOTE: compareTo is already handled by the `PartialOrd` and `Ord` derives.
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn empty_text_exception() {
		assert!(matches!(Slug::from_value("".to_string()), Err(IllegalArgument(_))))
	}
}
