use crate::collect_equal_element;
use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogy::weights::Weights;
use crate::helpers::collector::Collectors;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::option_extensions::OptionExtensions;
use crate::helpers::stream::Stream;
use crate::post::Post;
use crate::throw;

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
			throw!(IllegalArgumentException(format!(
				"Score should be in interval [0; 100]: {:?}",
				relation
			)));
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
				|posts, score| {
					posts
						.map(|ps| Relation::new(ps.post1, ps.post2, score.round() as i64))
						.transpose()
				},
			))?
			.or_else_throw(|| IllegalArgumentException("Can't create relation from zero typed relations.".into()))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::post::test::PostTestHelper;
	use literally::hmap;

	struct RelationTests {
		tag_weight: f64,
		link_weight: f64,
		tag_relation: RelationType,
		link_relation: RelationType,
		weights: Weights,
	}

	impl RelationTests {
		fn new() -> Result<RelationTests, Exception> {
			let tag_weight = 1.0;
			let link_weight = 1.0;
			let tag_relation = RelationType::new("tag".to_string())?;
			let link_relation = RelationType::new("link".to_string())?;
			Ok(Self {
				tag_weight,
				link_weight,
				tag_relation: tag_relation.clone(),
				link_relation: link_relation.clone(),
				weights: Weights::new(
					&hmap! {
						tag_relation => tag_weight,
						link_relation => link_weight,
					},
					0.5,
				),
			})
		}

		fn single_typed_relation_weight_one_same_posts_and_score(&self) -> Result<(), Exception> {
			let score = 60;
			let (post_a, post_b) = test_posts();
			let typed_relations = [TypedRelation::new(
				post_a.clone(),
				post_b.clone(),
				self.tag_relation.clone(),
				score,
			)?];

			let relation = Relation::aggregate(Stream::of(typed_relations), self.weights.clone())?;
			assert_eq!(post_a, relation.post1);
			assert_eq!(post_b, relation.post2);
			assert_eq!(score, relation.score);
			Ok(())
		}

		fn two_typed_relations_with_one_averaged_score(&self) -> Result<(), Exception> {
			let (post_a, post_b) = test_posts();
			let typed_relations = [
				TypedRelation::new(post_a.clone(), post_b.clone(), self.tag_relation.clone(), 40)?,
				TypedRelation::new(post_a, post_b, self.tag_relation.clone(), 80)?,
			];

			let relation = Relation::aggregate(Stream::of(typed_relations), self.weights.clone())?;
			assert_eq!((40 + 80) / 2, relation.score);
			Ok(())
		}

		fn two_typed_relations_differing_weight_weighted_score(&self) -> Result<(), Exception> {
			let (post_a, post_b) = test_posts();
			let typed_relations = [
				TypedRelation::new(post_a.clone(), post_b.clone(), self.tag_relation.clone(), 40)?,
				TypedRelation::new(post_a, post_b, self.link_relation.clone(), 80)?,
			];

			let relation = Relation::aggregate(Stream::of(typed_relations), self.weights.clone())?;
			let expected_score = ((40.0 * self.tag_weight + 80.0 * self.link_weight) / 2.0) as i64;
			assert_eq!(expected_score, relation.score);
			Ok(())
		}
	}

	#[test]
	fn single_typed_relation_weight_one_same_posts_and_score() {
		RelationTests::new()
			.unwrap()
			.single_typed_relation_weight_one_same_posts_and_score()
			.unwrap();
	}

	#[test]
	fn two_typed_relations_with_one_averaged_score() {
		RelationTests::new()
			.unwrap()
			.two_typed_relations_with_one_averaged_score()
			.unwrap();
	}

	#[test]
	fn two_typed_relations_differing_weight_weighted_score() {
		RelationTests::new()
			.unwrap()
			.two_typed_relations_differing_weight_weighted_score()
			.unwrap();
	}

	fn test_posts() -> (Post, Post) {
		let post_a = PostTestHelper::create_with_slug("a").unwrap();
		let post_b = PostTestHelper::create_with_slug("b").unwrap();
		(post_a, post_b)
	}
}
