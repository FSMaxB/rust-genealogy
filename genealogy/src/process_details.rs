pub fn process_details() -> String {
	// NOTE: The rust version is not known at runtime. Rust is natively compiled after all.
	format!("Process ID: {} | Rust version: unknown", std::process::id())
}
