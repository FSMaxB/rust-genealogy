use crate::helpers::exception::Exception;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn remove_outer_quotation_marks(string: &str) -> String {
	lazy_static! {
		static ref OUTER_QUOTATION_MARK_REGEX: Regex = Regex::new(r#"^"|"$"#).unwrap();
	}

	OUTER_QUOTATION_MARK_REGEX.replace_all(string, "").into_owned()
}

pub fn unchecked_files_read_all_lines(file: &PathBuf) -> Result<Vec<String>, Exception> {
	let file = File::open(file).map_err(Exception::from)?;
	BufReader::new(file)
		.lines()
		// NOTE: Collecting into `Result` allows collecting the Vec and isolating the exception case at the same time.
		// This works because `Result` implements the `FromIterator` trait, short circuiting on the first error it encounters.
		.collect::<Result<_, _>>()
		.map_err(Exception::from)
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
