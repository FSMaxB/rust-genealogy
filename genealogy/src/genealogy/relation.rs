use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::iterator::IteratorExtension;
use crate::helpers::mean::Mean;
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

	pub(crate) fn aggregate<'relations>(
		typed_relations: impl Iterator<Item = &'relations TypedRelation>,
		weights: &Weights,
	) -> Result<Relation, Exception> {
		// FIXME: Sometimes plain imperative code is just better than trying to do it the functional way!
		let (posts_iterator, score_iterator) = typed_relations
			.map(|relation| {
				(
					(&relation.post1, &relation.post2),
					(relation.score(), &relation.r#type),
				)
			})
			// NOTE: Split the `Iterator` into two instead of what the `tee` collector in Java does.
			.split_pair();
		// NOTE: Replacement for `collectEqual`
		let posts_iterator = posts_iterator.equal();
		let score_iterator =
			score_iterator.map(|(score, relation_type)| (score as f64) * weights.weight_of(relation_type));

		let (posts, mean) = posts_iterator
			.zip(score_iterator)
			.map(|(result, score)| result.map(|posts| (posts, score)))
			// NOTE: `averagingDouble` was replaced by `Mean`
			.try_fold((None, Mean::default()), |(_, mean), result| {
				let (posts, score) = result?;
				Ok::<_, Exception>((Some(posts), mean.add_number(score)))
			})?;
		let (posts, score) = posts
			.zip(Option::<f64>::from(mean))
			.ok_or_else(|| IllegalArgumentException("Can't create relation from zero typed relations.".to_string()))?;
		Ok(Relation {
			post1: posts.0.clone(),
			post2: posts.1.clone(),
			score: score as i64,
		})
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
										tag_relation.clone() => tag_weight,
										link_relation.clone() => link_weight,
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

			let relation = Relation::aggregate(typed_relations.iter(), &self.weights)?;
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

			let relation = Relation::aggregate(typed_relations.iter(), &self.weights)?;
			assert_eq!((40 + 80) / 2, relation.score);
			Ok(())
		}

		fn two_typed_relations_differing_weight_weighted_score(&self) -> Result<(), Exception> {
			let (post_a, post_b) = test_posts();
			let typed_relations = [
				TypedRelation::new(post_a.clone(), post_b.clone(), self.tag_relation.clone(), 40)?,
				TypedRelation::new(post_a, post_b, self.link_relation.clone(), 80)?,
			];

			let relation = Relation::aggregate(typed_relations.iter(), &self.weights)?;
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
