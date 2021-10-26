use genealogy_java_apis::stream::Stream;
use genealogy_java_apis::string::JString;

/// ```java
/// @FunctionalInterface
/// public interface Content extends Supplier<Stream<String>> {}
/// ```
pub type Content = Box<dyn FnOnce() -> Stream<JString>>;

pub trait ContentExtensions {
	fn get(self) -> Stream<JString>;
}

impl ContentExtensions for Content {
	fn get(self) -> Stream<JString> {
		self()
	}
}
