use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn remove_outer_quotation_marks(text: &str) -> String {
	lazy_static! {
		static ref QUOTATION_MARK_REGEX: Regex = Regex::new(r#"^"|"$"#).unwrap();
	}

	QUOTATION_MARK_REGEX.replace_all(text, "").into()
}

pub fn file_lines(path: &Path) -> Box<dyn Iterator<Item = Result<String, std::io::Error>>> {
	match File::open(path) {
		Ok(file) => Box::new(BufReader::new(file).lines()),
		Err(io_error) => Box::new(Err(io_error).into_iter()),
	}
}

#[cfg(test)]
mod test {
	use crate::exception::Exception;
	use crate::text_parser_tests::{test_text_parser, TextParser};
	use crate::utils::remove_outer_quotation_marks;

	struct QuotationParser {}

	impl TextParser for QuotationParser {
		fn parse_create_extract(text: String) -> Result<String, Exception> {
			Ok(remove_outer_quotation_marks(&text))
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<QuotationParser>()
	}
}
