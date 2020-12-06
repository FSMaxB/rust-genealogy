use crate::java_replicas::exception::Exception;
use crate::java_replicas::exception::Exception::IllegalArgument;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug {
	pub value: String,
}

impl Slug {
	#[allow(dead_code)]
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
