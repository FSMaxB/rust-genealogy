use crate::genealogy::score::{Score, WeightedScore};
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use std::convert::TryFrom;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Weight(f64);

impl TryFrom<f64> for Weight {
	type Error = Exception;

	fn try_from(weight: f64) -> Result<Self, Self::Error> {
		if !(0.0..=1.0).contains(&weight) {
			Err(IllegalArgument(format!(
				"Weight must be in interval [0; 1]: {}",
				weight
			)))
		} else {
			Ok(Weight(weight))
		}
	}
}

impl From<Weight> for f64 {
	fn from(weight: Weight) -> Self {
		weight.0
	}
}

impl Mul<Score> for Weight {
	type Output = WeightedScore;

	fn mul(self, score: Score) -> Self::Output {
		score * self
	}
}

impl PartialEq<f64> for Weight {
	fn eq(&self, other: &f64) -> bool {
		self.0 == *other
	}
}

impl PartialEq<Weight> for f64 {
	fn eq(&self, other: &Weight) -> bool {
		*self == other.0
	}
}

#[cfg(test)]
pub fn weight(weight: f64) -> Weight {
	Weight::try_from(weight).unwrap()
}
