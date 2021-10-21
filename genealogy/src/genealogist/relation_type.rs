use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string_extensions::StringExtensions;
use crate::throw;

/// ```java
/// public record RelationType(String value) {
///
/// 	// `RelationType` is a string (and not an enum) because {@code Genealogist} implementations
/// 	// can be plugged in via services, which means their type is unknown at runtime.
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationType {
	pub value: String,
}

impl RelationType {
	/// ```java
	/// public RelationType {
	///		requireNonNull(value);
	///		if (value.isBlank())
	///			throw new IllegalArgumentException("Relation types can't have an empty value.");
	///	}
	/// ```
	pub fn new(value: String) -> Result<RelationType, Exception> {
		if value.is_blank() {
			throw!(IllegalArgumentException(
				"Relation types can't have an empty value.".to_string()
			));
		}

		Ok(RelationType { value })
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
		assert_that(|| RelationType::new("".to_string()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
