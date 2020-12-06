use lazy_static::lazy_static;
use regex::Regex;

pub fn remove_outer_quotation_marks(string: &str) -> String {
	lazy_static! {
		static ref OUTER_QUOTATION_MARK_REGEX: Regex = Regex::new(r#"^"|"$"#).unwrap();
	}

	OUTER_QUOTATION_MARK_REGEX.replace_all(string, "").into_owned()
}

#[cfg(test)]
mod test {
	use crate::text_parser_tests::test_text_parser;
	use crate::utils::remove_outer_quotation_marks;

	#[test]
	fn quotation_tests() {
		test_text_parser(remove_outer_quotation_marks);
	}
}
