use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use std::convert::TryFrom;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Score(u8);

impl TryFrom<u8> for Score {
	type Error = Exception;

	fn try_from(score: u8) -> Result<Self, Self::Error> {
		if score > 100 {
			Err(IllegalArgument(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)))
		} else {
			Ok(Score(score))
		}
	}
}

impl TryFrom<f64> for Score {
	type Error = Exception;

	fn try_from(score: f64) -> Result<Self, Self::Error> {
		if score > 100.0 {
			Err(IllegalArgument(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)))
		} else {
			Ok(Score(score.round() as u8))
		}
	}
}

impl Mul<f64> for Score {
	type Output = f64;

	fn mul(self, rhs: f64) -> Self::Output {
		(self.0 as f64) * rhs
	}
}
