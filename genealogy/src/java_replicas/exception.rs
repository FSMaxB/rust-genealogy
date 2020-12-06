use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Exception {
	IllegalArgument(String),
	UncheckedIo(tokio::io::Error),
}

impl Display for Exception {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		use Exception::*;
		match self {
			IllegalArgument(message) => {
				write!(formatter, "IllegalArgumentException: '{}'", message)
			}
			UncheckedIo(error) => {
				write!(formatter, "UncheckedIOException: '{}'", error)
			}
		}
	}
}

impl Error for Exception {}

impl From<tokio::io::Error> for Exception {
	fn from(tokio_error: tokio::io::Error) -> Self {
		Self::UncheckedIo(tokio_error)
	}
}
