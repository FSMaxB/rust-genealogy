use genealogy_java_apis::stream::Stream;
use genealogy_java_apis::string::JString;
use std::rc::Rc;

/// ```java
/// @FunctionalInterface
/// public interface Content extends Supplier<Stream<String>> {}
/// ```
#[derive(Clone)]
pub struct Content {
	get: Rc<dyn Fn() -> Stream<JString>>,
}

impl Content {
	pub fn get(self) -> Stream<JString> {
		(self.get.as_ref())()
	}
}

// NOTE: In java this is automatically implemented
impl<Function> From<Function> for Content
where
	Function: Fn() -> Stream<JString> + 'static,
{
	fn from(function: Function) -> Self {
		Self { get: Rc::new(function) }
	}
}
