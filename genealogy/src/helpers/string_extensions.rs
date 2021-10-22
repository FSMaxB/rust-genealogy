use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use regex::{Regex, Replacer};

pub trait StringExtensions {
	fn replace_all<Replacement: regex::Replacer>(
		&self,
		regex: &'static str,
		replacement: Replacement,
	) -> Result<String, Exception>;

	fn split(&self, separator: char) -> List<String>;

	fn split_limit(&self, separator: char, limit: usize) -> List<String>;

	fn strip(&self) -> &str;

	fn is_blank(&self) -> bool;
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

	fn split(&self, separator: char) -> List<String> {
		self.as_ref().split(separator).map(str::to_owned).collect()
	}

	fn split_limit(&self, separator: char, limit: usize) -> List<String> {
		self.as_ref().splitn(limit, separator).map(str::to_owned).collect()
	}

	fn strip(&self) -> &str {
		self.as_ref().trim()
	}

	fn is_blank(&self) -> bool {
		self.as_ref().trim().is_empty()
	}
}
