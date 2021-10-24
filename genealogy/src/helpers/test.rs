use crate::helpers::exception::Exception;
use crate::helpers::stream::Streamable;
use std::cmp::PartialEq;
use std::fmt::Debug;

pub fn assert_that<Value>(value: Value) -> AssertThat<Value> {
	AssertThat { value }
}

pub struct AssertThat<Value> {
	value: Value,
}

impl<Value> AssertThat<Value> {
	#[track_caller]
	pub fn is_equal_to<RightHandSide>(&self, other: RightHandSide)
	where
		Value: PartialEq<RightHandSide> + Debug,
		RightHandSide: Debug,
	{
		if !self.value.eq(&other) {
			panic!("{:#?} is not equal to {:#?}", self.value, &other);
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

	pub fn extracting<Extracted>(self, extractor: impl FnMut(Value::Item) -> Extracted) -> AssertThat<Vec<Extracted>>
	where
		Value: IntoIterator,
	{
		AssertThat {
			value: self.value.into_iter().map(extractor).collect(),
		}
	}

	#[track_caller]
	pub fn contains(self, element: impl PartialEq<Value::Item> + Debug)
	where
		Value: IntoIterator,
		Value::Item: PartialEq + Debug,
	{
		for current_element in self.value {
			if element == current_element {
				return;
			}
		}
		panic!("Doesn't contain the element {:#?}", element);
	}

	#[track_caller]
	pub fn throws<OkType>(self) -> AssertThat<Exception>
	where
		Value: FnOnce() -> Result<OkType, Exception>,
		OkType: Debug,
	{
		match (self.value)() {
			Ok(value) => panic!("Didn't throw, got {:#?} instead", value),
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

impl<StreamableType> AssertThat<StreamableType>
where
	StreamableType: Streamable,
	StreamableType::Item: 'static,
{
	#[track_caller]
	pub fn contains_exactly_in_any_order<ExpectedValues>(self, expected_values: ExpectedValues)
	where
		ExpectedValues: IntoIterator,
		ExpectedValues::Item: Debug + PartialEq<StreamableType::Item>,
		StreamableType::Item: Debug,
	{
		let expected_values = expected_values.into_iter().collect::<Vec<ExpectedValues::Item>>();
		let actual_values = self
			.value
			.into_stream()
			.into_iterator()
			.collect::<Result<Vec<_>, _>>()
			.unwrap();

		if expected_values.len() != actual_values.len() {
			panic!(
				"The amount of values differs, expected {:#?}, got {:#?}.",
				expected_values, actual_values,
			)
		}

		'outer: for expected_value in &expected_values {
			for actual_value in &actual_values {
				if expected_value == actual_value {
					continue 'outer;
				}
			}
			panic!(
				"Didn't find expected value {:#?} in {:#?}",
				expected_value, actual_values
			);
		}
	}

	#[track_caller]
	pub fn contains_exactly<ExpectedValues>(self, expected_values: ExpectedValues)
	where
		ExpectedValues: IntoIterator,
		ExpectedValues::Item: Debug + PartialEq<StreamableType::Item>,
		StreamableType::Item: Debug,
	{
		let expected_values = expected_values.into_iter().collect::<Vec<ExpectedValues::Item>>();
		let actual_values = self
			.value
			.into_stream()
			.into_iterator()
			.collect::<Result<Vec<_>, _>>()
			.unwrap();

		if expected_values != actual_values {
			panic!(
				"The collections differ, actual: {:#?} expected: {:#?}",
				actual_values, expected_values
			);
		}
	}
}
