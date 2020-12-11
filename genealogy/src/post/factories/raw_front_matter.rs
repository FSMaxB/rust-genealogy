use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct RawFrontMatter {
	lines: BTreeMap<String, String>,
}

impl RawFrontMatter {
	pub fn new(lines: BTreeMap<String, String>) -> Self {
		Self { lines }
	}

	// NOTE: `valueOf` and `requiredValueOf` where combined since it doesn't make sense to have one with `Option` and one with `Result`.
	pub fn value_of(&self, key: &str) -> Result<&str, Exception> {
		self.lines
			.get(key)
			.map(String::as_str)
			.ok_or_else(|| IllegalArgument(format!("Required key '{}' not present in front matter.", key)))
	}
}
