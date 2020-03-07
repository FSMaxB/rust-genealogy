use crate::genealogist::relation_type::RelationType;
use std::collections::HashMap;

pub struct Weights {
	weights: HashMap<RelationType, f64>,
	default_weight: f64,
}

impl Weights {
	pub fn new(weights: HashMap<RelationType, f64>, default_weight: f64) -> Weights {
		Weights {
			weights,
			default_weight,
		}
	}

	pub fn all_equal() -> Weights {
		Weights {
			weights: Default::default(),
			default_weight: 1.0,
		}
	}

	pub fn weight_of(&self, genealogist_type: &RelationType) -> f64 {
		self.weights
			.get(genealogist_type)
			.copied()
			.unwrap_or(self.default_weight)
	}
}
