use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogy::score::Score;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::iterator::IteratorExtension;
use crate::helpers::mean::Mean;
use crate::post::Post;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Relation {
	pub post1: Arc<Post>,
	pub post2: Arc<Post>,
	pub score: Score,
}

impl Relation {
	pub(crate) fn aggregate<'relations>(
		typed_relations: impl Iterator<Item = &'relations TypedRelation>,
		weights: &Weights,
	) -> Result<Relation, Exception> {
		// FIXME: Sometimes plain imperative code is just better than trying to do it the functional way!
		let (posts_iterator, score_iterator) = typed_relations
			.map(|relation| {
				(
					(&relation.post1, &relation.post2),
					(relation.score, &relation.relation_type),
				)
			})
			// NOTE: Split the `Iterator` into two instead of what the `tee` collector in Java does.
			.split_pair();
		// NOTE: Replacement for `collectEqual`
		let posts_iterator = posts_iterator.equal();
		let score_iterator = score_iterator.map(|(score, relation_type)| score * weights.weight_of(relation_type));

		let (posts, mean) = posts_iterator
			.zip(score_iterator)
			.map(|(result, score)| result.map(|posts| (posts, score)))
			// NOTE: `averagingDouble` was replaced by `Mean`
			.try_fold((None, Mean::default()), |(_, mean), result| {
				let (posts, score) = result?;
				Ok::<_, Exception>((Some(posts), mean.add_number(score.into())))
			})?;
		let (posts, score) = posts
			.zip(Option::<f64>::from(mean))
			.ok_or_else(|| IllegalArgumentException("Can't create relation from zero typed relations.".to_string()))?;
		Ok(Relation {
			post1: posts.0.clone(),
			post2: posts.1.clone(),
			score: Score::try_from(score)?,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::score::score;
	use crate::genealogy::weight::{weight, Weight};
	use crate::post::test::post_with_slug;
	use lazy_static::lazy_static;
	use literally::bmap;
	use std::convert::TryInto;

	lazy_static! {
		static ref TAG_WEIGHT: Weight = weight(1.0);
		static ref LINK_WEIGHT: Weight = weight(1.0);
		static ref TAG_RELATION: RelationType = RelationType::from_value("tag".to_string()).unwrap();
		static ref LINK_RELATION: RelationType = RelationType::from_value("link".to_string()).unwrap();
		static ref WEIGHTS: Weights = Weights::new(
			bmap! {
				TAG_RELATION.clone() => *TAG_WEIGHT,
				LINK_RELATION.clone() => *LINK_WEIGHT,
			},
			weight(0.5),
		);
	}

	#[test]
	fn single_typed_relation_weight_one_same_posts_and_score() {
		let score = score(60);
		let (post_a, post_b) = test_posts();
		let typed_relations = [TypedRelation {
			post1: post_a.clone(),
			post2: post_b.clone(),
			relation_type: TAG_RELATION.clone(),
			score,
		}];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		assert_eq!(post_a, relation.post1);
		assert_eq!(post_b, relation.post2);
		assert_eq!(score, relation.score);
	}

	#[test]
	fn two_typed_relations_weith_one_averaged_score() {
		let (post_a, post_b) = test_posts();
		let typed_relations = [
			TypedRelation {
				post1: post_a.clone(),
				post2: post_b.clone(),
				relation_type: TAG_RELATION.clone(),
				score: score(40),
			},
			TypedRelation {
				post1: post_a,
				post2: post_b,
				relation_type: TAG_RELATION.clone(),
				score: score(80),
			},
		];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		assert_eq!(Score::try_from((40 + 80) / 2).unwrap(), relation.score);
	}

	#[test]
	fn two_typed_relations_differing_weight_weighted_score() {
		let (post_a, post_b) = test_posts();
		let typed_relations = [
			TypedRelation {
				post1: post_a.clone(),
				post2: post_b.clone(),
				relation_type: TAG_RELATION.clone(),
				score: 40.try_into().unwrap(),
			},
			TypedRelation {
				post1: post_a,
				post2: post_b,
				relation_type: LINK_RELATION.clone(),
				score: 80.try_into().unwrap(),
			},
		];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		let expected_score = vec![score(40) * *TAG_WEIGHT, score(80) * *LINK_WEIGHT]
			.into_iter()
			.collect::<Option<Score>>()
			.unwrap();
		assert_eq!(expected_score, relation.score);
	}

	fn test_posts() -> (Arc<Post>, Arc<Post>) {
		let post_a = post_with_slug("a").unwrap();
		let post_b = post_with_slug("b").unwrap();
		(Arc::new(post_a), Arc::new(post_b))
	}
}
