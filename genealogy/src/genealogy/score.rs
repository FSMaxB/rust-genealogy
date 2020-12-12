use crate::genealogy::weight::Weight;
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

impl Mul<Score> for f64 {
	type Output = f64;

	fn mul(self, score: Score) -> Self::Output {
		score * self
	}
}

impl From<WeightedScore> for f64 {
	fn from(weighted_score: WeightedScore) -> Self {
		weighted_score.0
	}
}

#[cfg(test)]
pub fn score(score: u8) -> Score {
	Score::try_from(score).unwrap()
}

pub struct WeightedScore(f64);

impl From<WeightedScore> for Score {
	fn from(weighted_score: WeightedScore) -> Self {
		Score(weighted_score.0.round() as u8)
	}
}

impl Mul<Weight> for Score {
	type Output = WeightedScore;

	fn mul(self, weight: Weight) -> Self::Output {
		WeightedScore((self.0 as f64) * f64::from(weight))
	}
}
