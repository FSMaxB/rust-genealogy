use crate::helpers::exception::Exception;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn assert_that<Value>(value: Value) -> AssertThat<Value> {
	AssertThat { value }
}

pub struct AssertThat<Value: ?Sized> {
	value: Value,
}

impl<Value> AssertThat<Value> {
	#[track_caller]
	pub fn is_equal_to<RightHandSide>(&self, other: &RightHandSide)
	where
		Value: PartialEq<RightHandSide> + Debug,
		RightHandSide: Debug + ?Sized,
	{
		if !self.value.eq(other) {
			panic!("{:?} is not equal to {:?}", self.value, other);
		}
	}

	#[track_caller]
	pub fn is_empty(self)
	where
		Value: IntoIterator,
	{
		let mut iterator = self.value.into_iter();
		if iterator.next().is_some() {
			panic!("The given value wasn't empty.")
		}
	}

	#[track_caller]
	pub fn contains(self, element: Value::Item)
	where
		Value: IntoIterator,
		<Value as IntoIterator>::Item: PartialEq + Debug,
	{
		for current_element in self.value {
			if element == current_element {
				return;
			}
		}
		panic!("Doesn't contain the element {:?}", element);
	}

	#[track_caller]
	pub fn throws<OkType>(self) -> AssertThat<Exception>
	where
		Value: FnOnce() -> Result<OkType, Exception>,
		OkType: Debug,
	{
		match (self.value)() {
			Ok(value) => panic!("Didn't throw, got {:?} instead", value),
			Err(exception) => AssertThat { value: exception },
		}
	}

	#[track_caller]
	pub fn and_satisfies(self, predicate: impl FnOnce(Value) -> bool) {
		if !predicate(self.value) {
			panic!("Value doesn't satisfy the given predicate.");
		}
	}
}
