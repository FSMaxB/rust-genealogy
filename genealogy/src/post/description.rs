use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string_extensions::StringExtensions;
use crate::throw;
use crate::utils::Utils;

/// ```java
/// public record Description(String text) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Description {
	pub text: String,
}

impl Description {
	/// ```java
	/// public Description {
	///		requireNonNull(text);
	///		text = Utils.removeOuterQuotationMarks(text).strip();
	///		if (text.isBlank())
	///			throw new IllegalArgumentException("Description can't have an empty text.");
	///	}
	/// ```
	pub fn new(text: &str) -> Result<Description, Exception> {
		let text = Utils::remove_outer_quotation_marks(text)?;
		if text.is_blank() {
			throw!(IllegalArgumentException(
				"Description can't have an empty text.".to_string()
			));
		}

		Ok(Description { text })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::{test_text_parser, QuotationTests};

	impl QuotationTests for Description {
		fn parse_create_extract(text: &str) -> Result<String, Exception> {
			Ok(Description::new(text)?.text)
		}
	}

	#[test]
	fn empty_text_exception() {
		assert!(matches!(Description::new(""), Err(IllegalArgumentException(_))))
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<Description>();
	}
}
