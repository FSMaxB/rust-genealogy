use genealogy_java_apis::function::Supplier;
use genealogy_java_apis::stream::Stream;
use genealogy_java_apis::string::JString;

/// ```java
/// @FunctionalInterface
/// public interface Content extends Supplier<Stream<String>> {}
/// ```
pub type Content = Supplier<Stream<JString>>;
