use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string_extensions::StringExtensions;
use crate::throw;
use crate::utils::Utils;

/// ```java
/// public record Title(String text) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Title {
	pub text: String,
}

impl Title {
	/// ```java
	/// public Title {
	///		requireNonNull(text);
	///		var unquotedText = Utils.removeOuterQuotationMarks(text);
	///		if (unquotedText.isBlank())
	///			throw new IllegalArgumentException("Titles can't have an empty text.");
	///		text = unquotedText;
	///	}
	/// ```
	pub fn new(text: &str) -> Result<Title, Exception> {
		let unquoted_text = Utils::remove_outer_quotation_marks(text)?;
		if unquoted_text.is_blank() {
			throw!(IllegalArgumentException("Titles can't have an empty text.".to_string()));
		}

		Ok(Title { text: unquoted_text })
	}
}

/// ``java
/// class TitleTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::exception::Exception::IllegalArgumentException;
	use crate::helpers::test::assert_that;
	use crate::text_parser_tests::{self, test_text_parser};

	/// ```java
	/// @Test
	///	void emptyText_exception() {
	///		assertThatThrownBy(() -> new Title("")).isInstanceOf(IllegalArgumentException.class);
	///	}
	/// ```
	#[test]
	fn empty_text_exception() {
		assert_that(|| Title::new(""))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}

	/// ```java
	/// @Nested
	///	class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	struct QuotationTests;

	/// ```java
	/// @Nested
	///	class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	impl text_parser_tests::QuotationTests for QuotationTests {
		/// ```java
		/// @Override
		///	public String parseCreateExtract(String text) {
		///		return new Title(text).text();
		///	}
		/// ```
		fn parse_create_extract(text: &str) -> Result<String, Exception> {
			Ok(Title::new(text)?.text)
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<QuotationTests>();
	}
}
