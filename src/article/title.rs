use crate::exception::Exception;
use crate::utils::remove_outer_quotation_marks;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Title {
	pub text: String,
}

impl TryFrom<String> for Title {
	type Error = Exception;

	fn try_from(text: String) -> Result<Self, Self::Error> {
		let unquoted_text = remove_outer_quotation_marks(&text);
		if unquoted_text.trim().is_empty() {
			Err(Exception::IllegalArgument(
				"Titles can't have an empty text.".to_string(),
			))
		} else {
			Ok(Self { text })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::{test_text_parser, TextParser};

	#[test]
	fn empty_text_exception() {
		let exception = Title::try_from(String::from("")).unwrap_err();
		match exception {
			Exception::IllegalArgument(_) => (),
			_ => panic!("Not an IllegalArgument exception"),
		}
	}

	struct QuotationParser {}

	impl TextParser for QuotationParser {
		fn parse_create_extract(text: String) -> Result<String, Exception> {
			Title::try_from(text).map(|title| title.text)
		}
	}

	fn quotation_tests() {
		test_text_parser::<QuotationParser>()
	}
}
