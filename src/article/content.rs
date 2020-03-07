use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

pub type ContentClosure = Box<dyn Fn() -> Box<dyn Iterator<Item = Result<String, std::io::Error>>>>;

/// Content wrapper type to allow implementing Debug
pub struct Content(ContentClosure);

impl<Closure, Iter> From<Closure> for Content
where
	Closure: Fn() -> Iter + 'static,
	Iter: Iterator<Item = Result<String, std::io::Error>> + 'static,
{
	fn from(closure: Closure) -> Self {
		Self(Box::new(move || {
			Box::new(closure()) as Box<dyn Iterator<Item = Result<String, std::io::Error>>>
		}))
	}
}

impl Deref for Content {
	type Target = ContentClosure;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for Content {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Debug for Content {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		formatter.write_str("fn () -> Iterator<Item = Result<String, std::io::Error>>")
	}
}
