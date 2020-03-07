use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Exception {
	IllegalArgument(String),
	Runtime(String),
	IO(std::io::Error),
	NullPointer,
	DateTimeParse,
}

impl Display for Exception {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		match self {
			Exception::IllegalArgument(message) => writeln!(formatter, "IllegalArgumentException: {}", message),
			Exception::Runtime(message) => writeln!(formatter, "RuntimeException: {}", message),
			Exception::IO(error) => writeln!(formatter, "IOException: {}", error),
			Exception::NullPointer => formatter.write_str("NullPointerException"),
			Exception::DateTimeParse => formatter.write_str("DateTimeParseException"),
		}
	}
}

impl Error for Exception {}

impl From<std::io::Error> for Exception {
	fn from(error: std::io::Error) -> Self {
		Exception::IO(error)
	}
}
