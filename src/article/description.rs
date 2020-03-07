use crate::exception::Exception;
use crate::utils::remove_outer_quotation_marks;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Description {
	pub text: String,
}

impl TryFrom<String> for Description {
	type Error = Exception;

	fn try_from(text: String) -> Result<Self, Self::Error> {
		let unquoted_text = remove_outer_quotation_marks(&text).trim().to_string();
		if unquoted_text.is_empty() {
			Err(Exception::IllegalArgument(
				"Description can't have an empty text.".to_string(),
			))
		} else {
			Ok(Self { text: unquoted_text })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::{test_text_parser, TextParser};

	#[test]
	fn empty_text_exception() {
		let exception = Description::try_from(String::from("")).unwrap_err();
		match exception {
			Exception::IllegalArgument(_) => (),
			_ => panic!("Not an IllegalArgument exception"),
		}
	}

	struct QuotationParser {}

	impl TextParser for QuotationParser {
		fn parse_create_extract(text: String) -> Result<String, Exception> {
			Description::try_from(text).map(|description| description.text)
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<QuotationParser>()
	}
}
