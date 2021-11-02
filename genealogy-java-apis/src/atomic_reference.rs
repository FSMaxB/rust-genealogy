use crate::exception::Exception;

pub struct AtomicReference<T> {
	// NOTE: The original code doesn't actually run concurrently, so no Atomics necessary
	value: Option<T>,
}

impl<T> AtomicReference<T> {
	#[allow(clippy::new_without_default)]
	pub fn new() -> Result<Self, Exception> {
		Ok(Self { value: None })
	}

	pub fn get(&self) -> Option<T>
	where
		T: Clone,
	{
		self.value.clone()
	}

	pub fn set(&mut self, value: T) {
		self.value.replace(value);
	}
}
