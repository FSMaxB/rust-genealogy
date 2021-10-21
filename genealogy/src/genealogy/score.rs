use crate::genealogy::weight::Weight;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::mean::Mean;
use std::iter::FromIterator;
use std::ops::Mul;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Score(u8);

impl TryFrom<u8> for Score {
	type Error = Exception;

	fn try_from(score: u8) -> Result<Self, Self::Error> {
		if score > 100 {
			Err(IllegalArgumentException(format!(
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
			Err(IllegalArgumentException(format!(
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

pub struct WeightedScore(f64);

impl From<WeightedScore> for i64 {
	fn from(weighted_score: WeightedScore) -> Self {
		weighted_score.0.round() as i64
	}
}

impl Mul<Weight> for Score {
	type Output = WeightedScore;

	fn mul(self, weight: Weight) -> Self::Output {
		WeightedScore((self.0 as f64) * f64::from(weight))
	}
}

impl Mul<Weight> for i64 {
	type Output = WeightedScore;

	fn mul(self, weight: Weight) -> Self::Output {
		WeightedScore((self as f64) * f64::from(weight))
	}
}

impl FromIterator<WeightedScore> for Option<i64> {
	fn from_iter<WeightedScoreIterator: IntoIterator<Item = WeightedScore>>(iterator: WeightedScoreIterator) -> Self {
		let mean = iterator.into_iter().map(f64::from).collect::<Mean>();
		Option::<f64>::from(mean)
			// clamping to [0; 1] to account for possible floating point inaccuracies
			.map(|value| value.max(0.0).min(100.0))
			.map(WeightedScore)
			.map(i64::from)
	}
}
