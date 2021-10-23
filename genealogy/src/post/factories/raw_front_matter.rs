use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::map::Map;
use crate::helpers::optional::Optional;
use crate::helpers::string::JString;

/// ```java
/// class RawFrontMatter {
///
/// 	private final Map<String, String> lines;
/// ```
#[derive(Debug)]
pub(super) struct RawFrontMatter {
	lines: Map<JString, JString>,
}

impl RawFrontMatter {
	/// ```java
	/// RawFrontMatter(Map<String, String> lines) {
	///		this.lines = lines;
	///	}
	/// ```
	pub(super) fn new(lines: Map<JString, JString>) -> Self {
		Self { lines }
	}

	/// ```java
	/// public Optional<String> valueOf(String key) {
	///		return Optional.ofNullable(lines.get(key));
	///	}
	/// ```
	pub fn value_of(&self, key: JString) -> Optional<JString> {
		Optional::of_nullable(self.lines.get(key))
	}

	/// ```java
	/// public String requiredValueOf(String key) {
	///		return valueOf(key).orElseThrow(
	///				() -> new IllegalArgumentException("Required key '" + key + "' not present in front matter."));
	///	}
	/// ```
	pub fn required_value_of(&self, key: JString) -> Result<JString, Exception> {
		self.value_of(key.clone())
			.or_else_throw(|| IllegalArgumentException("Required key '" + key + "' not present in front matter."))
	}
}
