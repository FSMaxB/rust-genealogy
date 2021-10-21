use crate::helpers::exception::Exception::{self, SecurityException};
use crate::throw;
use std::ffi::OsString;

pub enum System {}

impl System {
	pub fn get_property(key: &'static str) -> Result<OsString, Exception> {
		match key {
			"user.dir" => std::env::current_dir()
				.map_err(Into::into)
				.map(|path| path.into_os_string()),
			"user.home" => dirs::home_dir()
				.ok_or(SecurityException)
				.map(|path| path.into_os_string()),
			_ => throw!(SecurityException),
		}
	}
}
