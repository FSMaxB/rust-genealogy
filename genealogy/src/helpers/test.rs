use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn assert_that<Value>(value: &Value) -> AssertThat<'_, Value> {
	AssertThat { value }
}

pub struct AssertThat<'value, Value: ?Sized> {
	value: &'value Value,
}

impl<'value, Value> AssertThat<'value, Value>
where
	Value: Debug,
{
	#[track_caller]
	pub fn is_equal_to<RightHandSide>(&self, other: &RightHandSide)
	where
		Value: PartialEq<RightHandSide>,
		RightHandSide: Debug + ?Sized,
	{
		if !self.value.eq(other) {
			panic!("{:?} is not equal to {:?}", self.value, other);
		}
	}
}
