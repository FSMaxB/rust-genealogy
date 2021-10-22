use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::option_extensions::OptionExtensions;
use std::collections::HashMap;

/// ```java
/// class RawFrontMatter {
///
/// 	private final Map<String, String> lines;
/// ```
#[derive(Debug)]
pub struct RawFrontMatter {
	lines: HashMap<String, String>,
}

impl RawFrontMatter {
	/// ```java
	/// RawFrontMatter(Map<String, String> lines) {
	///		this.lines = lines;
	///	}
	/// ```
	pub fn new(lines: HashMap<String, String>) -> Self {
		Self { lines }
	}

	/// ```java
	/// public Optional<String> valueOf(String key) {
	///		return Optional.ofNullable(lines.get(key));
	///	}
	/// ```
	pub fn value_of(&self, key: &str) -> Option<String> {
		self.lines.get(key).map(Clone::clone)
	}

	/// ```java
	/// public String requiredValueOf(String key) {
	///		return valueOf(key).orElseThrow(
	///				() -> new IllegalArgumentException("Required key '" + key + "' not present in front matter."));
	///	}
	/// ```
	pub fn required_value_of(&self, key: &str) -> Result<String, Exception> {
		self.value_of(key)
			.or_else_throw(|| IllegalArgumentException(format!("Required key '{}' not present in front matter.", key)))
	}
}
