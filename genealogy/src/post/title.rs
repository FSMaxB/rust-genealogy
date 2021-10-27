use crate::utils::Utils;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::IllegalArgumentException;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::{record, throw};

/// ```java
/// public record Title(String text) {
/// ```
#[record(constructor = false)]
pub struct Title {
	text: JString,
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
	pub fn new(text: JString) -> Result<Title, Exception> {
		let unquoted_text = Utils::remove_outer_quotation_marks(text)?;
		if unquoted_text.is_blank() {
			throw!(IllegalArgumentException("Titles can't have an empty text.".into()));
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
	use crate::text_parser_tests::{self, test_text_parser};
	use genealogy_java_apis::exception::Exception::IllegalArgumentException;
	use genealogy_java_apis::test::assert_that;

	/// ```java
	/// @Test
	///	void emptyText_exception() {
	///		assertThatThrownBy(() -> new Title("")).isInstanceOf(IllegalArgumentException.class);
	///	}
	/// ```
	#[test]
	pub(super) fn empty_text_exception() {
		assert_that(|| Title::new("".into()))
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
		fn parse_create_extract(text: JString) -> Result<JString, Exception> {
			Ok(Title::new(text)?.text)
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<QuotationTests>();
	}
}
