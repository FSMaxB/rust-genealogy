use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Exception {
	IllegalArgument(String),
	UncheckedIO(std::io::Error),
	RuntimeException(String),
}

impl Display for Exception {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		use Exception::*;
		match self {
			IllegalArgument(message) => {
				write!(formatter, "IllegalArgumentException: '{}'", message)
			}
			UncheckedIO(error) => {
				write!(formatter, "UncheckedIOException: '{}'", error)
			}
			RuntimeException(message) => {
				write!(formatter, "RuntimeException: '{}'", message)
			}
		}
	}
}

impl Error for Exception {}

impl From<std::io::Error> for Exception {
	fn from(io_error: std::io::Error) -> Self {
		Self::UncheckedIO(io_error)
	}
}
