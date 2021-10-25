use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use crate::helpers::stream::Stream;
use regex::{Regex, Replacer};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref};
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

	pub fn replace(&self, target: &str, replacement: impl AsRef<str>) -> JString {
		self.text.as_ref().replace(target, replacement.as_ref()).into()
	}

	pub fn replace_all<Replacement: Replacer>(
		&self,
		regex: &'static str,
		replacement: Replacement,
	) -> Result<JString, Exception> {
		let regex = Regex::new(regex)?;
		Ok(regex.replace_all(self.text.as_ref(), replacement).as_ref().into())
	}

	pub fn to_lower_case(&self) -> Self {
		self.text.to_lowercase().into()
	}

	pub fn chars(&self) -> Stream<i32> {
		#[allow(clippy::needless_collect)] // it is needed because of lifetimes
		let utf16_code_units = self.text.encode_utf16().collect::<Vec<_>>();
		utf16_code_units
			.into_iter()
			.map(i32::from)
			.map(Result::<_, Exception>::Ok)
			.into()
	}
}

pub fn jstrings<'a>(iterable: impl IntoIterator<Item = &'a str>) -> List<JString> {
	let vector = iterable.into_iter().map(JString::from).collect::<Vec<_>>();
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

impl AsRef<std::path::Path> for JString {
	fn as_ref(&self) -> &std::path::Path {
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

impl<Displayable> Add<Displayable> for JString
where
	Displayable: Display,
{
	type Output = JString;

	fn add(self, right_hand_side: Displayable) -> Self::Output {
		format!("{}{}", self, right_hand_side).into()
	}
}

impl Add<JString> for &str {
	type Output = JString;

	fn add(self, right_hand_side: JString) -> Self::Output {
		format!("{}{}", self, right_hand_side).into()
	}
}
