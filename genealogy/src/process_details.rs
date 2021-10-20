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
	pub fn details() -> String {
		format!(
			"Process ID: {} | Rust version: {}",
			std::process::id(),
			rustc_version_runtime::version()
		)
	}
}
