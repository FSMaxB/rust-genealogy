use crate::helpers::exception::Exception;
use regex::{Regex, Replacer};

pub trait StringExtensions {
	fn replace_all<Replacement: regex::Replacer>(
		&self,
		regex: &'static str,
		replacement: Replacement,
	) -> Result<String, Exception>;

	fn split(&self, separator: char) -> Vec<String>;

	fn strip(&self) -> String;
}

impl<StringType> StringExtensions for StringType
where
	StringType: AsRef<str>,
{
	fn replace_all<Replacement: Replacer>(
		&self,
		regex: &'static str,
		replacement: Replacement,
	) -> Result<String, Exception> {
		let regex = Regex::new(regex)?;
		Ok(regex.replace_all(self.as_ref(), replacement).into_owned())
	}

	fn split(&self, separator: char) -> Vec<String> {
		self.as_ref().split(separator).map(str::to_owned).collect()
	}

	fn strip(&self) -> String {
		self.as_ref().trim().to_string()
	}
}
