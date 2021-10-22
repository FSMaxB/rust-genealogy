use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use regex::{Regex, Replacer};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JString {
	text: Arc<String>,
}

impl JString {
	pub fn is_empty(&self) -> bool {
		self.text.is_empty()
	}

	pub fn is_blank(&self) -> bool {
		self.text.as_ref().trim().is_empty()
	}

	pub fn starts_with<Prefix: AsRef<str>>(&self, prefix: Prefix) -> bool {
		self.text.as_ref().starts_with(prefix.as_ref())
	}

	pub fn strip(&self) -> JString {
		self.text.as_ref().trim().into()
	}

	pub fn split(&self, separator: char) -> List<JString> {
		self.text.as_ref().split(separator).map(JString::from).collect()
	}

	pub fn split_limit(&self, separator: char, limit: usize) -> List<JString> {
		self.text.as_ref().splitn(limit, separator).map(JString::from).collect()
	}

	pub fn replace_all<Replacement: Replacer>(
		&self,
		regex: &'static str,
		replacement: Replacement,
	) -> Result<JString, Exception> {
		let regex = Regex::new(regex)?;
		Ok(regex.replace_all(self.text.as_ref(), replacement).as_ref().into())
	}
}

pub fn jstrings<const N: usize>(array: [&str; N]) -> [JString; N] {
	let vector = array.into_iter().map(JString::from).collect::<Vec<_>>();
	vector.try_into().unwrap()
}

impl Display for JString {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		self.text.deref().fmt(formatter)
	}
}

impl PartialEq<str> for JString {
	fn eq(&self, other: &str) -> bool {
		self.text.as_ref() == other
	}
}

impl PartialEq<&str> for JString {
	fn eq(&self, other: &&str) -> bool {
		self.text.as_ref() == other
	}
}

impl PartialEq<JString> for str {
	fn eq(&self, other: &JString) -> bool {
		other.text.as_ref() == self
	}
}

impl PartialEq<JString> for &str {
	fn eq(&self, other: &JString) -> bool {
		other.text.as_ref() == self
	}
}

impl PartialEq<&JString> for JString {
	fn eq(&self, other: &&JString) -> bool {
		self == *other
	}
}

impl PartialEq<JString> for &JString {
	fn eq(&self, other: &JString) -> bool {
		*self == other
	}
}

impl From<&str> for JString {
	fn from(text: &str) -> Self {
		Self {
			text: Arc::new(text.into()),
		}
	}
}

impl From<&JString> for JString {
	fn from(jstring: &JString) -> Self {
		jstring.clone()
	}
}

impl From<String> for JString {
	fn from(string: String) -> Self {
		Self { text: Arc::new(string) }
	}
}

impl AsRef<Path> for JString {
	fn as_ref(&self) -> &Path {
		self.text.as_ref().as_ref()
	}
}

impl AsRef<str> for JString {
	fn as_ref(&self) -> &str {
		self.text.as_ref()
	}
}

impl AsRef<[u8]> for JString {
	fn as_ref(&self) -> &[u8] {
		self.text.deref().as_ref()
	}
}

impl From<JString> for PathBuf {
	fn from(jstring: JString) -> Self {
		jstring.text.deref().into()
	}
}

impl Serialize for JString {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.text.as_ref().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for JString {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Self {
			text: Arc::new(String::deserialize(deserializer)?),
		})
	}
}
