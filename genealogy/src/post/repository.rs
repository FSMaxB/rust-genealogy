use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string::JString;
use crate::throw;

/// ```java
/// public record Repository(String identifier) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Repository {
	pub identifier: JString,
}

impl Repository {
	/// ```java
	/// public Repository {
	///		identifier = requireNonNull(identifier);
	///		if (identifier.isBlank())
	///			throw new IllegalArgumentException("Repositories can't have an empty identifier.");
	///	}
	/// ```
	pub fn new(identifier: JString) -> Result<Repository, Exception> {
		if identifier.is_blank() {
			throw!(IllegalArgumentException(
				"Repositories can't have an empty identifier.".to_string()
			));
		}

		Ok(Repository { identifier })
	}
}

/// No original Java code here, but identical to the slug tests.
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;

	#[test]
	fn empty_text__exception() {
		assert_that(|| Repository::new("".into()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
