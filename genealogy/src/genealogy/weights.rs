use crate::genealogist::relation_type::RelationType;
use crate::helpers::map::{Map, MapExtension};
use crate::map_of;
use std::collections::HashMap;

/// ```java
/// public class Weights {
///
/// 	private final Map<RelationType, Double> weights;
/// 	private final double defaultWeight;
/// ```
#[derive(Debug)]
pub struct Weights {
	weights: HashMap<RelationType, f64>,
	default_weight: f64,
}

impl Weights {
	/// ```java
	/// public Weights(Map<RelationType, Double> weights, double defaultWeight) {
	///		this.weights = Map.copyOf(weights);
	///		this.defaultWeight = defaultWeight;
	///	}
	/// ```
	pub fn new(weights: &HashMap<RelationType, f64>, default_weight: f64) -> Self {
		Self {
			weights: Map::copy_of(weights),
			default_weight,
		}
	}

	/// ```java
	/// public static Weights allEqual() {
	///		return new Weights(Map.of(), 1);
	///	}
	/// ```
	pub fn all_equal() -> Self {
		Weights::new(&map_of!(), 1.0)
	}

	pub fn weight_of(&self, genealogist_type: &RelationType) -> f64 {
		self.weights.get_or_default(genealogist_type, self.default_weight)
	}
}

#[cfg(test)]
mod test {
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::weights::Weights;
	use lazy_static::lazy_static;
	use literally::hmap;

	lazy_static! {
		static ref TAG_TYPE: RelationType = RelationType::new("tag".to_string()).unwrap();
		static ref LIST_TYPE: RelationType = RelationType::new("list".to_string()).unwrap();
	}

	// NOTE: The following tests are omitted because there is no `null` in rust:
	// * nullRelationType_throwsException
	// * nullWeight_throwsException

	#[test]
	fn known_relation_type_returns_weight() {
		let weights = Weights::new(&hmap! {TAG_TYPE.clone() => 0.42}, 0.5);
		assert_eq!(0.42, weights.weight_of(&TAG_TYPE));
	}

	#[test]
	fn unknown_relation_type_returns_default_weight() {
		let weights = Weights::new(&hmap! {TAG_TYPE.clone() => 0.42}, 0.5);
		assert_eq!(0.5, weights.weight_of(&LIST_TYPE));
	}
}
