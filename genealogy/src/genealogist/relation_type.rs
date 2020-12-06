use crate::java_replicas::exception::Exception;
use crate::java_replicas::exception::Exception::IllegalArgument;

// TODO: Should this be wrapped in an Arc when using it?
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationType {
	pub value: String,
}

impl RelationType {
	#[allow(dead_code)]
	pub fn from_value(value: String) -> Result<RelationType, Exception> {
		if value.is_empty() {
			Err(IllegalArgument("Relation types can't have an empty value.".to_string()))
		} else {
			Ok(RelationType { value })
		}
	}
}
