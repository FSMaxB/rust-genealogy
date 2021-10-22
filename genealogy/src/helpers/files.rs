use crate::helpers::exception::Exception;
use crate::helpers::stream::Stream;
use resiter::Map;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

pub enum Files {}

impl Files {
	pub fn is_writable(path: &Path) -> bool {
		match path.metadata() {
			Ok(metadata) => !metadata.permissions().readonly(),
			Err(_) => false,
		}
	}

	pub fn read_all_lines(path: &Path) -> Result<Vec<String>, Exception> {
		let file = File::open(path)?;
		BufReader::new(file).lines().map_err(Exception::from).collect()
	}

	pub fn lines(path: &Path) -> Result<Stream<String>, Exception> {
		let file = File::open(path)?;
		Ok(BufReader::new(file).lines().map_err(Exception::from).into())
	}

	pub fn list(directory: &Path) -> Result<Stream<'static, PathBuf>, Exception> {
		Ok(directory.read_dir()?.map_ok(|dir_entry| dir_entry.path()).into())
	}

	pub fn write<'content>(path: &Path, lines: impl IntoIterator<Item = &'content str>) -> Result<(), Exception> {
		let file = OpenOptions::new().write(true).truncate(true).create(true).open(path)?;
		let mut writer = BufWriter::new(file);
		for line in lines {
			writer.write_all(line.as_bytes())?;
			writer.write_all(b"\n")?;
		}

		Ok(())
	}
}
