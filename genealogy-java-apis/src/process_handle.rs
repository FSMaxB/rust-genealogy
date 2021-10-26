pub struct ProcessHandle;

impl ProcessHandle {
	pub fn current() -> ProcessHandle {
		ProcessHandle
	}

	pub fn pid(&self) -> u32 {
		std::process::id()
	}
}
