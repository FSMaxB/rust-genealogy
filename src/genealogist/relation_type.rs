use crate::exception::Exception;
use std::convert::TryFrom;

pub struct RelationType {
	// Comment taken verbatim from the java implementation:
	// `RelationType` is a string (and not an enum) because Genealogist implementations
	// can be plugged in via services, which means their type is unknown at runtime.
	pub value: String,
}

impl TryFrom<String> for RelationType {
	type Error = Exception;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		if value.trim().is_empty() {
			Err(Exception::IllegalArgument(
				"Relation types can't have an empty value.".to_string(),
			))
		} else {
			Ok(RelationType { value })
		}
	}
}
