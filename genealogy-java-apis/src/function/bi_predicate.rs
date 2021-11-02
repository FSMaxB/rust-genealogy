use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone)]
pub struct BiPredicate<First, Second> {
	test: Rc<dyn Fn(First, Second) -> bool>,
}

impl<First, Second> BiPredicate<First, Second> {
	pub fn test(&self, first: First, second: Second) -> bool {
		(self.test)(first, second)
	}
}

impl<First, Second> Display for BiPredicate<First, Second> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "lambda${:p}", self.test)
	}
}

impl<First, Second> Debug for BiPredicate<First, Second> {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		<Self as Display>::fmt(self, formatter)
	}
}

impl<First, Second> PartialEq for BiPredicate<First, Second> {
	fn eq(&self, other: &Self) -> bool {
		#[allow(clippy::vtable_address_comparisons)]
		std::ptr::eq(self.test.as_ref(), other.test.as_ref())
	}
}

impl<First, Second> Hash for BiPredicate<First, Second> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.test.as_ref() as *const dyn Fn(First, Second) -> bool).hash(state)
	}
}

impl<First, Second> Eq for BiPredicate<First, Second> {}

impl<Function, First, Second> From<Function> for BiPredicate<First, Second>
where
	Function: Fn(First, Second) -> bool + 'static,
{
	fn from(function: Function) -> Self {
		Self {
			test: Rc::new(function),
		}
	}
}
