use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string::JString;
use crate::throw;
use crate::utils::Utils;

/// ```java
/// public record Description(String text) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Description {
	pub text: JString,
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
	pub fn new(text: JString) -> Result<Description, Exception> {
		let text = Utils::remove_outer_quotation_marks(text)?;
		if text.is_blank() {
			throw!(IllegalArgumentException("Description can't have an empty text.".into()));
		}

		Ok(Description { text })
	}
}

/// ```java
/// class DescriptionTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;
	use crate::text_parser_tests::{self, test_text_parser};

	/// ```java
	/// @Test
	///	void emptyText_exception() {
	///		assertThatThrownBy(() -> new Description("")).isInstanceOf(IllegalArgumentException.class);
	///	}
	/// ```
	#[test]
	pub(super) fn empty_text_exception() {
		assert_that(|| Description::new("".into()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}

	/// ```java
	/// 	@Nested
	///	class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	struct Quotationtests;

	/// ```java
	/// 	@Nested
	///	class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	impl text_parser_tests::QuotationTests for Quotationtests {
		/// ```java
		/// @Override
		///	public String parseCreateExtract(String text) {
		///		return new Description(text).text();
		///	}
		/// ```
		fn parse_create_extract(text: JString) -> Result<JString, Exception> {
			Ok(Description::new(text)?.text)
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<Quotationtests>();
	}
}
