use crate::genealogist::relation_type::RelationType;
use crate::helpers::map::{Map, MapExtension};
use crate::map_of;
use std::collections::HashMap;
use std::rc::Rc;

/// ```java
/// public class Weights {
///
/// 	private final Map<RelationType, Double> weights;
/// 	private final double defaultWeight;
/// ```
#[derive(Debug, Clone)]
pub struct Weights {
	weights: Rc<HashMap<RelationType, f64>>,
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
			weights: Rc::new(Map::copy_of(weights)),
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

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::weights::Weights;
	use crate::helpers::test::assert_that;
	use crate::map_of;
	use lazy_static::lazy_static;

	/// ```java
	/// class WeightsTests {
	/// ```
	pub struct WeightsTests;

	impl WeightsTests {
		/// ```java
		/// public static final RelationType TAG_TYPE = new RelationType("tag");
		/// ```
		fn tag_type() -> RelationType {
			lazy_static! {
				static ref TAG_TYPE: RelationType = RelationType::new("tag".into()).unwrap();
			};
			TAG_TYPE.clone()
		}

		/// ```java
		/// public static final RelationType LIST_TYPE = new RelationType("list");
		/// ```
		fn list_type() -> RelationType {
			lazy_static! {
				static ref LIST_TYPE: RelationType = RelationType::new("list".into()).unwrap();
			};
			LIST_TYPE.clone()
		}

		// NOTE: The following tests are omitted because there is no `null` in rust:
		//
		// 	@Test
		// 	void nullRelationType_throwsException() {
		// 		var weightMap = new HashMap<RelationType, Double>();
		// 		weightMap.put(null, 1.0);
		// 		assertThatThrownBy(() -> new Weights(weightMap, 0.5)).isInstanceOf(NullPointerException.class);
		// 	}
		//
		// 	@Test
		// 	void nullWeight_throwsException() {
		// 		var weightMap = new HashMap<RelationType, Double>();
		// 		weightMap.put(TAG_TYPE, null);
		// 		assertThatThrownBy(() -> new Weights(weightMap, 0.5)).isInstanceOf(NullPointerException.class);
		// 	}

		/// ```java
		///	@Test
		///	void knownRelationType_returnsWeight() {
		///		var weights = new Weights(Map.of(TAG_TYPE, 0.42), 0.5);
		///
		///		assertThat(weights.weightOf(TAG_TYPE)).isEqualTo(0.42);
		///	}
		/// ```
		fn known_relation_type__returns_weight() {
			let weights = Weights::new(&map_of!(Self::tag_type(), 0.42), 0.5);

			assert_that(weights.weight_of(&Self::tag_type())).is_equal_to(0.42);
		}

		/// ```java
		/// @Test
		///	void unknownRelationType_returnsDefaultWeight() {
		///		var weights = new Weights(Map.of(TAG_TYPE, 0.42), 0.5);
		///
		///		assertThat(weights.weightOf(LIST_TYPE)).isEqualTo(0.5);
		///	}
		/// ```
		fn unknown_relation_type__returns_default_weight() {
			let weights = Weights::new(&map_of!(Self::tag_type(), 0.42), 0.5);

			assert_that(weights.weight_of(&Self::list_type())).is_equal_to(0.5);
		}
	}

	#[test]
	fn known_relation_type__returns_weight() {
		WeightsTests::known_relation_type__returns_weight();
	}

	#[test]
	fn unknown_relation_type__returns_default_weight() {
		WeightsTests::unknown_relation_type__returns_default_weight();
	}
}
