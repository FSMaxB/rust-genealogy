use crate::java_replicas::exception::Exception;
use crate::java_replicas::exception::Exception::IllegalArgument;
use crate::utils::remove_outer_quotation_marks;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Description {
	pub text: String,
}

impl Description {
	#[allow(dead_code)]
	pub fn from_text(text: &str) -> Result<Description, Exception> {
		let unquoted_text = remove_outer_quotation_marks(text);
		if unquoted_text.trim().is_empty() {
			Err(IllegalArgument("Description can't have an empty text.".to_string()))
		} else {
			Ok(Description { text: unquoted_text })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::test_text_parser;

	#[test]
	fn empty_text_exception() {
		assert!(matches!(Description::from_text(""), Err(IllegalArgument(_))))
	}

	#[test]
	fn quotation_tests() {
		test_text_parser(|text| Description::from_text(text).unwrap().text)
	}
}
