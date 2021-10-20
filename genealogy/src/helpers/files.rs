use crate::helpers::exception::Exception;
use resiter::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Files;

impl Files {
	pub fn is_writable(path: &Path) -> bool {
		match path.metadata() {
			Ok(metadata) => !metadata.permissions().readonly(),
			Err(_) => false,
		}
	}

	pub fn read_all_lines(path: &Path) -> Result<Stream<String>, Exception> {
		let file = File::open(path)?;
		Ok(BufReader::new(file).lines().map_err(Exception::from).into())
	}
}
