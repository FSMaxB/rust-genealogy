use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::test::assert_that;

/// ```java
/// public interface QuotationTests {
/// ```
#[allow(non_snake_case)]
pub trait QuotationTests {
	/// ```java
	/// String parseCreateExtract(String text);
	/// ```
	fn parse_create_extract(text: JString) -> Result<JString, Exception>;

	/// ```java
	/// @Test
	///	default void createFromStringWithoutQuotationMarks_noChange() {
	///		var text = "A cool blog post";
	///		var expected = text;
	///
	///		var actual = parseCreateExtract(text);
	///
	///		assertThat(actual).isEqualTo(expected);
	///	}
	/// ```
	fn create_from_string_without_quotation_marks__no_change() {
		let text: JString = "A cool blog post".into();
		let expected = text.clone();

		let actual = Self::parse_create_extract(text).unwrap();

		assert_that(actual).is_equal_to(expected);
	}

	/// ```java
	///	@Test
	///	default void createFromStringWithQuotationMarks_quotationMarksRemoved() {
	///		var text = "\"A cool blog post\"";
	///		var expected = "A cool blog post";
	///
	///		var actual = parseCreateExtract(text);
	///
	///		assertThat(actual).isEqualTo(expected);
	///	}
	/// ```
	fn create_from_string_with_quotation_marks__quotation_marks_removed() {
		let text = r#""A cool blog post""#.into();
		let expected = "A cool blog post";

		let actual = Self::parse_create_extract(text).unwrap();

		assert_that(actual).is_equal_to(expected);
	}

	/// ```java
	///	@Test
	///	default void createFromStringWithInnerQuotationMarks_onlyOuterQuotationMarksRemoved() {
	///		var text = "\"\"A cool blog post\" he said\"";
	///		var expected = "\"A cool blog post\" he said";
	///
	///		var actual = parseCreateExtract(text);
	///
	///		assertThat(actual).isEqualTo(expected);
	///	}
	/// ```
	fn create_from_string_with_inner_quotation_marks__only_outer_quotation_marks_removed() {
		let text = r#""A cool blog post""#.into();
		let expected = "A cool blog post";

		let actual = Self::parse_create_extract(text).unwrap();

		assert_that(actual).is_equal_to(expected);
	}
}

pub fn test_text_parser<Test>()
where
	Test: QuotationTests,
{
	Test::create_from_string_without_quotation_marks__no_change();
	Test::create_from_string_with_quotation_marks__quotation_marks_removed();
	Test::create_from_string_with_inner_quotation_marks__only_outer_quotation_marks_removed();
}
