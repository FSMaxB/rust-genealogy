use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use crate::helpers::iterator::IteratorExtension;
use crate::helpers::mean::Mean;
use crate::post::Post;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Relation {
	pub post1: Arc<Post>,
	pub post2: Arc<Post>,
	pub score: u64,
}

impl Relation {
	#[allow(dead_code)]
	pub fn new(post1: Arc<Post>, post2: Arc<Post>, score: u64) -> Result<Relation, Exception> {
		// WTF: Why use a `long` if you are actually only allowing values up to 100?
		// RUSTIFICATION: Create a separate type that upholds this invariant.
		if score > 100 {
			Err(IllegalArgument(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)))
		} else {
			Ok(Relation { post1, post2, score })
		}
	}

	#[allow(dead_code)]
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
		let score_iterator =
			score_iterator.map(|(score, relation_type)| (score as f64) * weights.weight_of(relation_type));

		let (posts, mean) = posts_iterator
			.zip(score_iterator)
			.map(|(result, score)| result.map(|posts| (posts, score)))
			// NOTE: `averagingDouble` was replaced by `Mean`
			.try_fold((None, Mean::default()), |(_, mean), result| {
				let (posts, score) = result?;
				Ok::<_, Exception>((Some(posts), mean.add(score)))
			})?;
		let (posts, score) = posts
			.zip(Option::<f64>::from(mean))
			.ok_or_else(|| IllegalArgument("Can't create relation from zero typed relations.".to_string()))?;
		Ok(Relation::new(posts.0.clone(), posts.1.clone(), score.round() as u64)?)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::post::test::post_with_slug;
	use lazy_static::lazy_static;
	use literally::bmap;

	const TAG_WEIGHT: f64 = 1.0;
	const LINK_WEIGHT: f64 = 1.0;

	lazy_static! {
		static ref TAG_RELATION: RelationType = RelationType::from_value("tag".to_string()).unwrap();
		static ref LINK_RELATION: RelationType = RelationType::from_value("link".to_string()).unwrap();
		static ref WEIGHTS: Weights = Weights::new(
			bmap! {
				TAG_RELATION.clone() => TAG_WEIGHT,
				LINK_RELATION.clone() => LINK_WEIGHT,
			},
			0.5
		);
	}

	#[test]
	fn single_typed_relation_weight_one_same_posts_and_score() {
		let score = 60;
		let (post_a, post_b) = test_posts();
		let typed_relations =
			[TypedRelation::new(post_a.clone(), post_b.clone(), TAG_RELATION.clone(), score).unwrap()];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		assert_eq!(post_a, relation.post1);
		assert_eq!(post_b, relation.post2);
		assert_eq!(score, relation.score);
	}

	#[test]
	fn two_typed_relations_weith_one_averaged_score() {
		let (post_a, post_b) = test_posts();
		let typed_relations = [
			TypedRelation::new(post_a.clone(), post_b.clone(), TAG_RELATION.clone(), 40).unwrap(),
			TypedRelation::new(post_a, post_b, TAG_RELATION.clone(), 80).unwrap(),
		];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		assert_eq!((40 + 80) / 2, relation.score);
	}

	#[test]
	fn two_typed_relations_differing_weight_weighted_score() {
		let (post_a, post_b) = test_posts();
		let typed_relations = [
			TypedRelation::new(post_a.clone(), post_b.clone(), TAG_RELATION.clone(), 40).unwrap(),
			TypedRelation::new(post_a, post_b, LINK_RELATION.clone(), 80).unwrap(),
		];

		let relation = Relation::aggregate(typed_relations.iter(), &WEIGHTS).unwrap();
		let expected_score = ((40.0 * TAG_WEIGHT + 80.0 * LINK_WEIGHT) / 2.0).round() as u64;
		assert_eq!(expected_score, relation.score);
	}

	fn test_posts() -> (Arc<Post>, Arc<Post>) {
		let post_a = post_with_slug("a").unwrap();
		let post_b = post_with_slug("b").unwrap();
		(Arc::new(post_a), Arc::new(post_b))
	}
}
