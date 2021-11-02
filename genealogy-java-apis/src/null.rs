#[derive(PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct null;

impl<T> PartialEq<Option<T>> for null {
	fn eq(&self, other: &Option<T>) -> bool {
		other.is_none()
	}
}

impl<T> PartialEq<null> for Option<T> {
	fn eq(&self, _other: &null) -> bool {
		self.is_none()
	}
}
