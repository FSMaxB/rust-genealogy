use crate::collect_equal_element;
use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogy::weights::Weights;
use crate::helpers::collector::Collectors;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use crate::post::Post;
use crate::throw;
use std::fmt::{Display, Formatter};

/// ```java
/// public record Relation(
///		Post post1,
///		Post post2,
///		long score) {
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Relation {
	pub post1: Post,
	pub post2: Post,
	pub score: i64,
}

impl Relation {
	/// ```java
	///	public Relation {
	///		requireNonNull(post1);
	///		requireNonNull(post2);
	///		if (score < 0 || 100 < score)
	///			throw new IllegalArgumentException("Score should be in interval [0; 100]: " + toString());
	///	}
	/// ```
	pub fn new(post1: Post, post2: Post, score: i64) -> Result<Relation, Exception> {
		let relation = Relation { post1, post2, score };

		#[allow(clippy::manual_range_contains)]
		if (score < 0) || (100 < score) {
			throw!(IllegalArgumentException(
				JString::from("Score should be in interval [0; 100]: {:?}") + relation.to_string()
			));
		}

		Ok(relation)
	}

	/// ```java
	/// static Relation aggregate(Stream<TypedRelation> typedRelations, Weights weights) {
	///		record Posts(Post post1, Post post2) { }
	///		return typedRelations.collect(
	///				teeing(
	///						mapping(
	///								rel -> new Posts(rel.post1(), rel.post2()),
	///								collectEqualElement()),
	///						averagingDouble(rel -> rel.score() * weights.weightOf(rel.type())),
	///						(posts, score) -> posts.map(ps -> new Relation(ps.post1(), ps.post2(), round(score)))
	///				))
	///				.orElseThrow(() -> new IllegalArgumentException("Can't create relation from zero typed relations."));
	///	}
	/// ```
	pub(super) fn aggregate(typed_relations: Stream<TypedRelation>, weights: Weights) -> Result<Relation, Exception> {
		#[derive(Debug, PartialEq, Eq)]
		struct Posts {
			post1: Post,
			post2: Post,
		}

		impl Posts {
			fn new(post1: Post, post2: Post) -> Self {
				Self { post1, post2 }
			}
		}

		typed_relations
			.collect(Collectors::teeing(
				Collectors::mapping(
					|rel: TypedRelation| Posts::new(rel.post1, rel.post2),
					collect_equal_element!(),
				),
				Collectors::averaging_double(move |rel: TypedRelation| {
					Ok((rel.score() as f64) * weights.weight_of(&rel.r#type))
				}),
				|posts, score| posts.map(|ps| Relation::new(ps.post1, ps.post2, score.round() as i64)),
			))?
			.or_else_throw(|| IllegalArgumentException("Can't create relation from zero typed relations.".into()))
	}
}

impl Display for Relation {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		write!(formatter, "{:?}", self)
	}
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::helpers::test::assert_that;
	use crate::map_of;
	use crate::post::test::PostTestHelper;

	/// ```java
	/// class RelationTests {
	/// ```
	struct RelationTests {
		post_a: Post,
		post_b: Post,
		tag_relation: RelationType,
		link_relation: RelationType,
		weights: Weights,
	}

	impl RelationTests {
		/// ```java
		/// private static final double TAG_WEIGHT = 1.0;
		/// ```
		const TAG_WEIGHT: f64 = 1.0;
		/// ```java
		///	private static final double LINK_WEIGHT = 0.25;
		/// ```
		const LINK_WEIGHT: f64 = 1.0;

		/// ```java
		/// private final Post postA = PostTestHelper.createWithSlug("a");
		///	private final Post postB = PostTestHelper.createWithSlug("b");
		///
		///	private final RelationType tagRelation = new RelationType("tag");
		///	private final RelationType linkRelation = new RelationType("link");
		///
		///	private final Weights weights = new Weights(
		///			Map.of(
		///					tagRelation, TAG_WEIGHT,
		///					linkRelation, LINK_WEIGHT),
		///			0.5);
		/// ```
		fn new() -> Result<RelationTests, Exception> {
			let tag_relation = RelationType::new("tag".into())?;
			let link_relation = RelationType::new("link".into())?;
			Ok(Self {
				post_a: PostTestHelper::create_with_slug("a".into())?,
				post_b: PostTestHelper::create_with_slug("b".into())?,
				tag_relation: tag_relation.clone(),
				link_relation: link_relation.clone(),
				weights: Weights::new(
					&map_of!(tag_relation, Self::TAG_WEIGHT, link_relation, Self::LINK_WEIGHT,),
					0.5,
				),
			})
		}

		/// ```java
		/// 	@Test
		///	void singleTypedRelation_weightOne_samePostsAndScore() {
		///		int score = 60;
		///		var typedRelations = Stream.of(
		///				new TypedRelation(postA, postB, tagRelation, score)
		///		);
		///
		///		var relation = Relation.aggregate(typedRelations, weights);
		///
		///		assertThat(relation.post1()).isEqualTo(postA);
		///		assertThat(relation.post2()).isEqualTo(postB);
		///		assertThat(relation.score()).isEqualTo(score);
		///	}
		/// ```
		pub(super) fn single_typed_relation__weight_one__same_posts_and_score(&self) -> Result<(), Exception> {
			let score = 60;
			let typed_relations = Stream::of([TypedRelation::new(
				self.post_a.clone(),
				self.post_b.clone(),
				self.tag_relation.clone(),
				score,
			)?]);

			let relation = Relation::aggregate(typed_relations, self.weights.clone())?;

			assert_that(&relation.post1).is_equal_to(&self.post_a);
			assert_that(&relation.post2).is_equal_to(&self.post_b);
			assert_that(relation.score).is_equal_to(score);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void twoTypedRelation_weightOne_averagedScore() {
		///		var typedRelations = Stream.of(
		///				new TypedRelation(postA, postB, tagRelation, 40),
		///				new TypedRelation(postA, postB, tagRelation, 80)
		///		);
		///
		///		var relation = Relation.aggregate(typedRelations, weights);
		///
		///		assertThat(relation.score()).isEqualTo((40 + 80) / 2);
		///	}
		/// ```
		pub(super) fn two_typed_relations__weight_one__averaged_score(&self) -> Result<(), Exception> {
			let typed_relations = Stream::of([
				TypedRelation::new(self.post_a.clone(), self.post_b.clone(), self.tag_relation.clone(), 40)?,
				TypedRelation::new(self.post_a.clone(), self.post_b.clone(), self.tag_relation.clone(), 80)?,
			]);

			let relation = Relation::aggregate(typed_relations, self.weights.clone())?;

			assert_that(relation.score).is_equal_to((40 + 80) / 2);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void twoTypedRelation_differingWeight_weightedScore() {
		///		var typedRelations = Stream.of(
		///				new TypedRelation(postA, postB, tagRelation, 40),
		///				new TypedRelation(postA, postB, linkRelation, 80)
		///		);
		///
		///		var relation = Relation.aggregate(typedRelations, weights);
		///
		///		double expectedScore = (40 * TAG_WEIGHT + 80 * LINK_WEIGHT) / 2;
		///		assertThat(relation.score()).isEqualTo(round(expectedScore));
		///	}
		/// ```
		pub(super) fn two_typed_relation__differing_weight__weighted_score(&self) -> Result<(), Exception> {
			let typed_relations = Stream::of([
				TypedRelation::new(self.post_a.clone(), self.post_b.clone(), self.tag_relation.clone(), 40)?,
				TypedRelation::new(self.post_a.clone(), self.post_b.clone(), self.link_relation.clone(), 80)?,
			]);

			let relation = Relation::aggregate(typed_relations, self.weights.clone())?;

			let expected_score = (40.0 * Self::TAG_WEIGHT + 80.0 * Self::LINK_WEIGHT) / 2.0;
			assert_that(relation.score).is_equal_to(expected_score.round() as i64);
			Ok(())
		}
	}

	#[test]
	fn single_typed_relation__weight_one__same_posts_and_score() {
		RelationTests::new()
			.unwrap()
			.single_typed_relation__weight_one__same_posts_and_score()
			.unwrap();
	}

	#[test]
	fn two_typed_relations__weight_one__averaged_score() {
		RelationTests::new()
			.unwrap()
			.two_typed_relations__weight_one__averaged_score()
			.unwrap();
	}

	#[test]
	fn two_typed_relation__differing_weight__weighted_score() {
		RelationTests::new()
			.unwrap()
			.two_typed_relation__differing_weight__weighted_score()
			.unwrap();
	}
}
