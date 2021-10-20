use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Repository {
	pub identifier: String,
}

impl Repository {
	pub fn from_identifier(identifier: String) -> Result<Repository, Exception> {
		if identifier.trim().is_empty() {
			Err(IllegalArgumentException(
				"Repositories can't have an empty identifier.".to_string(),
			))
		} else {
			Ok(Repository { identifier })
		}
	}
}
