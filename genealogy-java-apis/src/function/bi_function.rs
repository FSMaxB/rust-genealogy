use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct BiFunction<First, Second, Output> {
	apply: Rc<dyn Fn(First, Second) -> Output>,
}

impl<First, Second, Output> BiFunction<First, Second, Output> {
	pub fn apply(&self, first: First, second: Second) -> Output {
		(self.apply)(first, second)
	}
}

impl<First, Second, Output> Display for BiFunction<First, Second, Output> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "lambda${:p}", self.apply)
	}
}

impl<First, Second, Output> Debug for BiFunction<First, Second, Output> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		<Self as Display>::fmt(self, formatter)
	}
}

impl<First, Second, Output> PartialEq for BiFunction<First, Second, Output> {
	fn eq(&self, other: &Self) -> bool {
		(self.apply.as_ref() as *const dyn Fn(First, Second) -> Output)
			== (other.apply.as_ref() as *const dyn Fn(First, Second) -> Output)
	}
}

impl<First, Second, Output> Hash for BiFunction<First, Second, Output> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.apply.as_ref() as *const dyn Fn(First, Second) -> Output).hash(state)
	}
}

impl<First, Second, Output> Eq for BiFunction<First, Second, Output> {}

impl<Function, First, Second, Output> From<Function> for BiFunction<First, Second, Output>
where
	Function: Fn(First, Second) -> Output + 'static,
{
	fn from(function: Function) -> Self {
		Self {
			apply: Rc::new(function),
		}
	}
}
