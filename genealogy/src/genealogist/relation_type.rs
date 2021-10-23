use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string::JString;
use crate::throw;

/// ```java
/// public record RelationType(String value) {
///
/// 	// `RelationType` is a string (and not an enum) because {@code Genealogist} implementations
/// 	// can be plugged in via services, which means their type is unknown at runtime.
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RelationType {
	pub value: JString,
}

impl RelationType {
	/// ```java
	/// public RelationType {
	///		requireNonNull(value);
	///		if (value.isBlank())
	///			throw new IllegalArgumentException("Relation types can't have an empty value.");
	///	}
	/// ```
	pub fn new(value: JString) -> Result<RelationType, Exception> {
		if value.is_blank() {
			throw!(IllegalArgumentException(
				"Relation types can't have an empty value.".into()
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
		assert_that(|| RelationType::new("".into()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
