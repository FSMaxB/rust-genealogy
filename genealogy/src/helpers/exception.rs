use std::error::Error;
use std::fmt::{Display, Formatter};
use url::ParseError;

/// Fake exception type to emulate Java exceptions used in the original
#[derive(Debug)]
pub enum Exception {
	IllegalArgumentException(String),
	UncheckedIO(std::io::Error),
	RuntimeException(String, Box<dyn Error>),
	DateTimeException(chrono::format::ParseError),
	IndexOutOfBoundsException(usize),
	PatternSyntaxException(regex::Error),
	SecurityException,
	URISyntaxException(url::ParseError),
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
			RuntimeException(message, exception) => {
				write!(formatter, "RuntimeException: '{}', cause: {}", message, exception)
			}
			DateTimeException(error) => {
				write!(formatter, "DateTimeException: '{}'", error)
			}
			IndexOutOfBoundsException(index) => {
				write!(formatter, "IndexOutOfBoundsException: {}", index)
			}
			PatternSyntaxException(error) => {
				write!(formatter, "PatternSyntaxException: {}", error)
			}
			SecurityException => formatter.write_str("SecurityException"),
			URISyntaxException(error) => {
				write!(formatter, "URISyntaxException: {}", error)
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

impl From<chrono::format::ParseError> for Exception {
	fn from(parse_error: chrono::format::ParseError) -> Self {
		Self::DateTimeException(parse_error)
	}
}

impl From<regex::Error> for Exception {
	fn from(regex_error: regex::Error) -> Self {
		Self::PatternSyntaxException(regex_error)
	}
}

impl From<url::ParseError> for Exception {
	fn from(parse_error: ParseError) -> Self {
		Self::URISyntaxException(parse_error)
	}
}

/// Fake `throw` to emulate `throw new Exception` in Java
#[macro_export]
macro_rules! throw {
	($exception:expr) => {
		return Err($exception)
	};
}
