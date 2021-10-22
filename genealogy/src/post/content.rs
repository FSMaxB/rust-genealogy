use crate::helpers::stream::Stream;
use crate::helpers::string::JString;

/// ```java
/// @FunctionalInterface
/// public interface Content extends Supplier<Stream<String>> {}
/// ```
pub type Content = Box<dyn FnOnce() -> Stream<'static, JString>>;
