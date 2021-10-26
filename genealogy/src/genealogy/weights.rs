use crate::genealogist::relation_type::RelationType;
use genealogy_java_apis::map::Map;
use genealogy_java_apis::map_of;

/// ```java
/// public class Weights {
///
/// 	private final Map<RelationType, Double> weights;
/// 	private final double defaultWeight;
/// ```
#[derive(Debug, Clone)]
pub struct Weights {
	weights: Map<RelationType, f64>,
	default_weight: f64,
}

impl Weights {
	/// ```java
	/// public Weights(Map<RelationType, Double> weights, double defaultWeight) {
	///		this.weights = Map.copyOf(weights);
	///		this.defaultWeight = defaultWeight;
	///	}
	/// ```
	pub fn new(weights: Map<RelationType, f64>, default_weight: f64) -> Self {
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
		Weights::new(map_of!(), 1.0)
	}

	pub fn weight_of(&self, genealogist_type: RelationType) -> f64 {
		self.weights.get_or_default(genealogist_type, self.default_weight)
	}
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::weights::Weights;
	use genealogy_java_apis::test::assert_that;
	use genealogy_java_apis::{map_of, r#static};

	/// ```java
	/// class WeightsTests {
	/// ```
	pub struct WeightsTests;

	impl WeightsTests {
		// ```java
		// public static final RelationType TAG_TYPE = new RelationType("tag");
		// ```
		r#static!(pub TAG_TYPE: RelationType = RelationType::new("tag".into()).unwrap());

		// ```java
		// public static final RelationType LIST_TYPE = new RelationType("list");
		// ```
		r#static!(pub LIST_TYPE: RelationType = RelationType::new("list".into()).unwrap());

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
		pub(super) fn known_relation_type__returns_weight() {
			let weights = Weights::new(map_of!(Self::TAG_TYPE(), 0.42), 0.5);

			assert_that(weights.weight_of(Self::TAG_TYPE())).is_equal_to(0.42);
		}

		/// ```java
		/// @Test
		///	void unknownRelationType_returnsDefaultWeight() {
		///		var weights = new Weights(Map.of(TAG_TYPE, 0.42), 0.5);
		///
		///		assertThat(weights.weightOf(LIST_TYPE)).isEqualTo(0.5);
		///	}
		/// ```
		pub(super) fn unknown_relation_type__returns_default_weight() {
			let weights = Weights::new(map_of!(Self::TAG_TYPE(), 0.42), 0.5);

			assert_that(weights.weight_of(Self::LIST_TYPE())).is_equal_to(0.5);
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
