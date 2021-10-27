use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct Supplier<Output> {
	get: Rc<dyn Fn() -> Output>,
}

impl<Output> Clone for Supplier<Output> {
	fn clone(&self) -> Self {
		Self { get: self.get.clone() }
	}
}

impl<Output> Supplier<Output> {
	pub fn get(&self) -> Output {
		(self.get)()
	}
}

impl<Output> Display for Supplier<Output> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "lambda${:p}", self.get)
	}
}

impl<Function, Output> From<Function> for Supplier<Output>
where
	Function: Fn() -> Output + 'static,
{
	fn from(function: Function) -> Self {
		Self { get: Rc::new(function) }
	}
}

impl<Output> Debug for Supplier<Output> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		<Self as Display>::fmt(self, formatter)
	}
}

impl<Output> PartialEq for Supplier<Output> {
	fn eq(&self, other: &Self) -> bool {
		(self.get.as_ref() as *const dyn Fn() -> Output) == (other.get.as_ref() as *const dyn Fn() -> Output)
	}
}

impl<Output> Hash for Supplier<Output> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.get.as_ref() as *const dyn Fn() -> Output).hash(state)
	}
}

impl<Output> Eq for Supplier<Output> {}
