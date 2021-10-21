pub enum Path {}

impl Path {
	pub fn of(path: &impl AsRef<std::path::Path>) -> &std::path::Path {
		path.as_ref()
	}
}
