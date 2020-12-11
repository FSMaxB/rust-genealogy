use crate::helpers::exception::Exception;
use crate::helpers::iterator::result_iterator::ResultIteratorExtension;
use lazy_static::lazy_static;
use regex::Regex;
use resiter::Map;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub fn remove_outer_quotation_marks(string: &str) -> String {
	lazy_static! {
		static ref OUTER_QUOTATION_MARK_REGEX: Regex = Regex::new(r#"^"|"$"#).unwrap();
	}

	OUTER_QUOTATION_MARK_REGEX.replace_all(string, "").into_owned()
}

pub fn unchecked_files_list(dir: &Path) -> impl Iterator<Item = Result<PathBuf, Exception>> {
	read_dir(dir)
		.into_result_iterator()
		.map(|result| result.and_then(std::convert::identity))
		.map(|result| result.map_err(Exception::from))
		.map_ok(|dir_entry| dir_entry.path())
}

pub fn unchecked_files_write(path: &Path, content: &str) -> Result<(), Exception> {
	Ok(File::create(path)?.write_all(content.as_bytes())?)
}

pub fn unchecked_files_read_all_lines(file: &Path) -> Result<Vec<String>, Exception> {
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
