use crate::genealogist::relation_type::RelationType;
use crate::genealogy::weight::Weight;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Weights {
	weights: HashMap<RelationType, Weight>,
	default_weight: Weight,
}

impl Weights {
	pub fn new(weights: HashMap<RelationType, Weight>, default_weight: Weight) -> Self {
		Self {
			weights,
			default_weight,
		}
	}

	pub fn all_equal() -> Self {
		Self {
			weights: Default::default(),
			default_weight: Weight::try_from(1.0).unwrap(),
		}
	}

	pub fn weight_of(&self, genealogist_type: &RelationType) -> Weight {
		self.weights
			.get(genealogist_type)
			.copied()
			.unwrap_or(self.default_weight)
	}
}

#[cfg(test)]
mod test {
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::weight::weight;
	use crate::genealogy::weights::Weights;
	use lazy_static::lazy_static;
	use literally::hmap;

	lazy_static! {
		static ref TAG_TYPE: RelationType = RelationType::from_value("tag".to_string()).unwrap();
		static ref LIST_TYPE: RelationType = RelationType::from_value("list".to_string()).unwrap();
	}

	// NOTE: The following tests are omitted because there is no `null` in rust:
	// * nullRelationType_throwsException
	// * nullWeight_throwsException

	#[test]
	fn known_relation_type_returns_weight() {
		let weights = Weights::new(hmap! {TAG_TYPE.clone() => weight(0.42)}, weight(0.5));
		assert_eq!(0.42, weights.weight_of(&TAG_TYPE));
	}

	#[test]
	fn unknown_relation_type_returns_default_weight() {
		let weights = Weights::new(hmap! {TAG_TYPE.clone() => weight(0.42)}, weight(0.5));
		assert_eq!(0.5, weights.weight_of(&LIST_TYPE));
	}
}
