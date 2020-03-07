#![cfg(test)]

use crate::exception::Exception;

pub trait TextParser {
	fn parse_create_extract(text: String) -> Result<String, Exception>;
}

pub fn test_text_parser<Parser: TextParser>() {
	create_from_string_without_quotation_marks__no_change::<Parser>();
	create_from_string_with_quotation_marks__quotation_marks_removed::<Parser>();
	create_from_string_with_inner_quotation_marks__only_outer_quotation_marks_removed::<Parser>();
}

#[allow(non_snake_case)]
fn create_from_string_without_quotation_marks__no_change<Parser: TextParser>() {
	let text = "A cool blog post";
	let expected = text;

	let actual = Parser::parse_create_extract(text.to_string()).unwrap();
	assert_eq!(expected, actual);
}

#[allow(non_snake_case)]
fn create_from_string_with_quotation_marks__quotation_marks_removed<Parser: TextParser>() {
	let text = r#""A cool blog post""#;
	let expected = "A cool blog post";

	let actual = Parser::parse_create_extract(text.to_string()).unwrap();
	assert_eq!(expected, actual);
}

#[allow(non_snake_case)]
fn create_from_string_with_inner_quotation_marks__only_outer_quotation_marks_removed<Parser: TextParser>() {
	let text = r#"""A cool blog post"""#;
	let expected = r#""A cool blog post""#;

	let actual = Parser::parse_create_extract(text.to_string()).unwrap();
	assert_eq!(expected, actual);
}
