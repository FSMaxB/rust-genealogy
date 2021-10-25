use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::NoSuchElementException;

#[derive(Clone, Debug, PartialEq)]
pub struct Optional<T>(Option<T>);

impl<T> Optional<T> {
	pub fn of(value: T) -> Self {
		Self(Some(value))
	}

	/// "nullable" is an Option in this case
	pub fn of_nullable(value: Option<T>) -> Self {
		Self(value)
	}

	pub fn empty() -> Self {
		Self(None)
	}

	pub fn is_present(&self) -> bool {
		self.0.is_some()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_none()
	}

	pub fn get(&self) -> Result<T, Exception>
	where
		T: Clone,
	{
		match &self.0 {
			None => Err(NoSuchElementException("No value present")),
			Some(value) => Ok(value.clone()),
		}
	}

	pub fn set(&mut self, value: T) {
		self.0.replace(value);
	}

	pub fn or_else_throw(self, thrower: impl FnOnce() -> Exception) -> Result<T, Exception> {
		self.0.ok_or_else(thrower)
	}

	pub fn map<NewValue>(
		self,
		mapper: impl FnOnce(T) -> Result<NewValue, Exception>,
	) -> Result<Optional<NewValue>, Exception> {
		self.0.map(mapper).transpose().map(Optional::from)
	}

	pub fn if_present(&self, action: impl FnOnce(T) -> Result<(), Exception>) -> Result<(), Exception>
	where
		T: Clone,
	{
		self.0.as_ref().cloned().map(action).unwrap_or(Ok(()))
	}

	pub fn if_present_or_else(
		&self,
		action: impl FnOnce(T) -> Result<(), Exception>,
		fallback_action: impl FnOnce(),
	) -> Result<(), Exception>
	where
		T: Clone,
	{
		self.0.as_ref().cloned().map(action).unwrap_or_else(|| {
			fallback_action();
			Ok(())
		})
	}

	pub fn as_ref(&self) -> Optional<&T> {
		self.0.as_ref().into()
	}
}

impl<T> From<Option<T>> for Optional<T> {
	fn from(option: Option<T>) -> Self {
		Optional(option)
	}
}

impl<T> IntoIterator for Optional<T> {
	type Item = T;
	type IntoIter = <Option<T> as IntoIterator>::IntoIter;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}
