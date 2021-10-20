use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::utils::Utils;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Description {
	pub text: String,
}

impl Description {
	pub fn from_text(text: &str) -> Result<Description, Exception> {
		let unquoted_text = Utils::remove_outer_quotation_marks(text)?;
		if unquoted_text.trim().is_empty() {
			Err(IllegalArgumentException(
				"Description can't have an empty text.".to_string(),
			))
		} else {
			Ok(Description { text: unquoted_text })
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::{test_text_parser, QuotationTests};

	impl QuotationTests for Description {
		fn parse_create_extract(text: &str) -> Result<String, Exception> {
			Ok(Description::from_text(text)?.text)
		}
	}

	#[test]
	fn empty_text_exception() {
		assert!(matches!(Description::from_text(""), Err(IllegalArgumentException(_))))
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<Description>();
	}
}
