use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Exception {
	IllegalArgument(String),
}

impl Display for Exception {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		match self {
			Exception::IllegalArgument(message) => {
				write!(formatter, "IllegalArgument: '{}'", message)
			}
		}
	}
}

impl Error for Exception {}
