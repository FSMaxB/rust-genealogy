use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::IllegalArgumentException;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::{record, throw};

/// ```java
/// public record Repository(String identifier) {
/// ```
#[record(constructor = false)]
pub struct Repository {
	identifier: JString,
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
				"Repositories can't have an empty identifier.".into()
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
	use genealogy_java_apis::test::assert_that;

	#[test]
	fn empty_text__exception() {
		assert_that(|| Repository::new("".into()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
