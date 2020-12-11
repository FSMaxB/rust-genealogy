use crate::genealogist::relation_type::RelationType;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Weights {
	weights: BTreeMap<RelationType, f64>,
	default_weight: f64,
}

impl Weights {
	pub fn new(weights: BTreeMap<RelationType, f64>, default_weight: f64) -> Self {
		Self {
			weights,
			default_weight,
		}
	}

	pub fn all_equal() -> Self {
		Self {
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

#[cfg(test)]
mod test {
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::weights::Weights;
	use lazy_static::lazy_static;
	use literally::bmap;

	lazy_static! {
		static ref TAG_TYPE: RelationType = RelationType::from_value("tag".to_string()).unwrap();
		static ref LIST_TYPE: RelationType = RelationType::from_value("list".to_string()).unwrap();
	}

	// NOTE: The following tests are omitted because there is no `null` in rust:
	// * nullRelationType_throwsException
	// * nullWeight_throwsException

	#[test]
	fn known_relation_type_returns_weight() {
		let weights = Weights::new(bmap! {TAG_TYPE.clone() => 0.42}, 0.5);
		assert_eq!(0.42, weights.weight_of(&TAG_TYPE));
	}

	#[test]
	fn unknown_relation_type_returns_default_weight() {
		let weights = Weights::new(bmap! {TAG_TYPE.clone() => 0.42}, 0.5);
		assert_eq!(0.5, weights.weight_of(&LIST_TYPE));
	}
}
