use crate::helpers::exception::Exception::{self, SecurityException};
use crate::helpers::string::JString;
use crate::throw;

pub enum System {}

impl System {
	pub fn get_property(key: &'static str) -> Result<JString, Exception> {
		match key {
			"user.dir" => std::env::current_dir()
				.map_err(Into::into)
				.map(|path| path.to_string_lossy().to_string()),
			"user.home" => dirs::home_dir()
				.ok_or(SecurityException)
				.map(|path| path.to_string_lossy().to_string()),
			_ => throw!(SecurityException),
		}
		.map(JString::from)
	}
}
