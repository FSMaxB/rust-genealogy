use genealogy_java_apis::process_handle::ProcessHandle;
use genealogy_java_apis::runtime::Runtime;
use genealogy_java_apis::string::JString;

/// ```java
/// public class ProcessDetails {
/// ```
pub struct ProcessDetails;

impl ProcessDetails {
	/// ```java
	/// public static String details() {
	/// 	return "Process ID: %s | Major Java version: %s".formatted(
	/// 			ProcessHandle.current().pid(),
	/// 			Runtime.version().major());
	/// }
	/// ```
	/// Note: Uses the full rust version, not just the major version,
	/// which up till now (2021) has only ever been 1.
	pub fn details() -> JString {
		format!(
			"Process ID: {} | Rust version: {}",
			ProcessHandle::current().pid(),
			Runtime::version(),
		)
		.into()
	}
}
