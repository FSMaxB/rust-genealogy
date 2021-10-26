use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::IllegalArgumentException;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::throw;

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
	use genealogy_java_apis::test::assert_that;

	#[test]
	fn empty_text__exception() {
		assert_that(|| RelationType::new("".into()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
