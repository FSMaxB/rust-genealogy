use std::fmt::{Display, Formatter};
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
