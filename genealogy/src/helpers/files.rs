use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use resiter::Map;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub enum Files {}

impl Files {
	pub fn exists(path: impl AsRef<Path>) -> bool {
		path.as_ref().exists()
	}

	pub fn is_writable(path: impl AsRef<Path>) -> bool {
		match path.as_ref().metadata() {
			Ok(metadata) => !metadata.permissions().readonly(),
			Err(_) => false,
		}
	}

	pub fn read_all_lines(path: impl AsRef<Path>) -> Result<List<JString>, Exception> {
		let file = File::open(path)?;
		BufReader::new(file)
			.lines()
			.map_ok(JString::from)
			.map_err(Exception::from)
			.collect()
	}

	pub fn lines(path: impl AsRef<Path>) -> Result<Stream<'static, JString>, Exception> {
		let file = File::open(path)?;
		Ok(BufReader::new(file)
			.lines()
			.map_ok(JString::from)
			.map_err(Exception::from)
			.into())
	}

	pub fn list(directory: impl AsRef<Path>) -> Result<Stream<'static, PathBuf>, Exception> {
		Ok(directory
			.as_ref()
			.read_dir()?
			.map_ok(|dir_entry| dir_entry.path())
			.into())
	}

	pub fn write<Lines, Line>(path: impl AsRef<Path>, lines: Lines) -> Result<(), Exception>
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
