use std::error::Error;
use std::fmt::{Display, Formatter};

/// Fake exception type to emulate Java exceptions used in the original
#[derive(Debug)]
pub enum Exception {
	IllegalArgumentException(String),
	UncheckedIO(std::io::Error),
	RuntimeException(String),
	DateTimeException(chrono::format::ParseError),
	IndexOutOfBoundsException(usize),
	SecurityException,
}

impl Display for Exception {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		use Exception::*;
		match self {
			IllegalArgumentException(message) => {
				write!(formatter, "IllegalArgumentException: '{}'", message)
			}
			UncheckedIO(error) => {
				write!(formatter, "UncheckedIOException: '{}'", error)
			}
			RuntimeException(message) => {
				write!(formatter, "RuntimeException: '{}'", message)
			}
			DateTimeException(error) => {
				write!(formatter, "DateTimeException: '{}'", error)
			}
			IndexOutOfBoundsException(index) => {
				write!(formatter, "IndexOutOfBoundsException: {}", index)
			}
			SecurityException => formatter.write_str("SecurityException"),
		}
	}
}

impl Error for Exception {}

impl From<std::io::Error> for Exception {
	fn from(io_error: std::io::Error) -> Self {
		Self::UncheckedIO(io_error)
	}
}

impl From<chrono::format::ParseError> for Exception {
	fn from(parse_error: chrono::format::ParseError) -> Self {
		Self::DateTimeException(parse_error)
	}
}

/// Fake `throw` to emulate `throw new Exception` in Java
#[macro_export]
macro_rules! throw {
	($exception:expr) => {
		return Err($exception);
	};
}
