use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use crate::helpers::path::Path;
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

pub enum Files {}

impl Files {
	pub fn exists(path: impl AsRef<std::path::Path>) -> bool {
		path.as_ref().exists()
	}

	pub fn is_directory(path: impl AsRef<std::path::Path>) -> bool {
		path.as_ref().is_dir()
	}

	pub fn is_regular_file(path: impl AsRef<std::path::Path>) -> bool {
		path.as_ref().is_file()
	}

	pub fn is_writable(path: impl AsRef<std::path::Path>) -> bool {
		match path.as_ref().metadata() {
			Ok(metadata) => !metadata.permissions().readonly(),
			Err(_) => false,
		}
	}

	pub fn read_all_lines(path: impl AsRef<std::path::Path>) -> Result<List<JString>, Exception> {
		let file = File::open(path)?;
		BufReader::new(file)
			.lines()
			.map(|result| result.map(JString::from).map_err(Exception::from))
			.collect()
	}

	pub fn lines(path: impl AsRef<std::path::Path>) -> Result<Stream<JString>, Exception> {
		let file = File::open(path)?;
		Ok(BufReader::new(file)
			.lines()
			.map(|result| result.map(JString::from).map_err(Exception::from))
			.into())
	}

	pub fn list(directory: impl AsRef<std::path::Path>) -> Result<Stream<Path>, Exception> {
		Ok(directory
			.as_ref()
			.read_dir()?
			.map(|result| result.map(|dir_entry| dir_entry.path().into()))
			.into())
	}

	pub fn write<Lines, Line>(path: impl AsRef<std::path::Path>, lines: Lines) -> Result<(), Exception>
	where
		Line: AsRef<[u8]>,
		Lines: IntoIterator<Item = Line>,
	{
		let file = OpenOptions::new().write(true).truncate(true).create(true).open(path)?;
		let mut writer = BufWriter::new(file);
		for line in lines {
			writer.write_all(line.as_ref())?;
			writer.write_all(b"\n")?;
		}

		Ok(())
	}
}
