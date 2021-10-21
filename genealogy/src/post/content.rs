use crate::helpers::stream::Stream;

/// ```java
/// @FunctionalInterface
/// public interface Content extends Supplier<Stream<String>> {}
/// ```
pub type Content = Box<dyn FnOnce() -> Stream<String> + Send + Sync>;
