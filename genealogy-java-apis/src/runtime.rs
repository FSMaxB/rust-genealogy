pub use rustc_version_runtime::Version;

pub enum Runtime {}

impl Runtime {
	pub fn version() -> Version {
		rustc_version_runtime::version()
	}
}
