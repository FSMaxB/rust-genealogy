use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use crate::utils::remove_outer_quotation_marks;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Title {
	pub text: String,
}

impl Title {
	pub fn from_text(text: &str) -> Result<Title, Exception> {
		let unquoted_text = remove_outer_quotation_marks(text);
		if unquoted_text.trim().is_empty() {
			Err(IllegalArgument("Titles can't have an empty text.".to_string()))
		} else {
			Ok(Title { text: unquoted_text })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::exception::Exception::IllegalArgument;
	use crate::text_parser_tests::test_text_parser;

	#[test]
	fn empty_text_exception() {
		assert!(matches!(Title::from_text(""), Err(IllegalArgument(_))))
	}

	#[test]
	fn quotation_tests() {
		test_text_parser(|text| Title::from_text(text).unwrap().text)
	}
}
