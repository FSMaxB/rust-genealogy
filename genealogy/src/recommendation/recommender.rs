use crate::genealogy::relation::Relation;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use crate::recommendation::Recommendation;
use resiter::Map;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

pub struct Recommender;

impl Recommender {
	#[allow(dead_code)]
	pub fn recommend(
		relations: impl Iterator<Item = Result<Relation, Exception>>,
		per_post: usize,
	) -> Result<impl Iterator<Item = Recommendation>, Exception> {
		// RUSTIFICATION: NonZeroUsize
		if per_post < 1 {
			return Err(IllegalArgument(format!(
				"Number of recommendations per post must be greater zero: {}",
				per_post
			)));
		}
		let by_post = relations
			.map_ok(RelationSortedByPostThenByDecreasingScore::from)
			.map_ok(|relation| (relation.post1.clone(), relation))
			.try_fold(BTreeMap::new(), |mut map, result| {
				let (post1, relation) = result?;
				// NOTE: The BTreeSet already puts the relations into a sorted order
				map.entry(post1).or_insert_with(BTreeSet::new).insert(relation);
				Ok::<_, Exception>(map)
			})?;

		Ok(by_post.into_iter().map(move |(post, sorted_relations)| {
			Recommendation::new(
				post,
				sorted_relations.into_iter().map(|relation| relation.post2.clone()),
				per_post,
			)
		}))
	}
}

#[derive(PartialEq, Eq)]
struct RelationSortedByPostThenByDecreasingScore(Relation);

impl Deref for RelationSortedByPostThenByDecreasingScore {
	type Target = Relation;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<Relation> for RelationSortedByPostThenByDecreasingScore {
	fn from(relation: Relation) -> Self {
		Self(relation)
	}
}

impl From<RelationSortedByPostThenByDecreasingScore> for Relation {
	fn from(relation: RelationSortedByPostThenByDecreasingScore) -> Self {
		relation.0
	}
}

impl PartialOrd for RelationSortedByPostThenByDecreasingScore {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for RelationSortedByPostThenByDecreasingScore {
	fn cmp(&self, other: &Self) -> Ordering {
		match self.0.post1.slug().cmp(other.0.post1.slug()) {
			Ordering::Equal => self.0.score.cmp(&other.0.score),
			ordering => ordering,
		}
		.reverse()
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::post::test::post_with_slug;
	use crate::post::Post;
	use lazy_static::lazy_static;
	use literally::bset;
	use std::sync::Arc;

	lazy_static! {
		static ref POST_A: Arc<Post> = Arc::new(post_with_slug("a").unwrap());
		static ref POST_B: Arc<Post> = Arc::new(post_with_slug("b").unwrap());
		static ref POST_C: Arc<Post> = Arc::new(post_with_slug("c").unwrap());
		static ref RELATION_AB: Relation = RelationTestHelper::create(POST_A.clone(), POST_B.clone(), 60).unwrap();
		static ref RELATION_AC: Relation = RelationTestHelper::create(POST_A.clone(), POST_C.clone(), 40).unwrap();
		static ref RELATION_BA: Relation = RelationTestHelper::create(POST_B.clone(), POST_A.clone(), 50).unwrap();
		static ref RELATION_BC: Relation = RelationTestHelper::create(POST_B.clone(), POST_C.clone(), 70).unwrap();
		static ref RELATION_CA: Relation = RelationTestHelper::create(POST_C.clone(), POST_A.clone(), 80).unwrap();
		static ref RELATION_CB: Relation = RelationTestHelper::create(POST_C.clone(), POST_B.clone(), 60).unwrap();
	}

	struct RelationTestHelper;

	impl RelationTestHelper {
		// WTF: Why a static method just to call the constructor with the exact same arguments. WTF x2
		fn create(post1: Arc<Post>, post2: Arc<Post>, score: u64) -> Result<Relation, Exception> {
			Relation::new(post1, post2, score)
		}
	}

	#[test]
	fn for_one_post_one_relation() {
		let recommendations = Recommender::recommend(vec![RELATION_AC.clone()].into_iter().map(Ok), 1)
			.unwrap()
			.collect::<BTreeSet<_>>();
		let expected_recommendations =
			bset! {Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_C.clone()]}};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[test]
	fn for_one_post_two_relations() {
		let recommendations =
			Recommender::recommend(vec![RELATION_AB.clone(), RELATION_AC.clone()].into_iter().map(Ok), 1)
				.unwrap()
				.collect::<BTreeSet<_>>();
		let expected_recommendations =
			bset! {Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_B.clone()]}};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[test]
	fn for_many_posts_one_relation_each() {
		let recommendations = Recommender::recommend(
			vec![RELATION_AC.clone(), RELATION_BC.clone(), RELATION_CB.clone()]
				.into_iter()
				.map(Ok),
			1,
		)
		.unwrap()
		.collect::<BTreeSet<_>>();
		let expected_recommendations = bset! {
			Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_B.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_C.clone(), recommended_posts: vec![POST_B.clone()]},
		};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[test]
	fn for_many_posts_two_relations_each() {
		let recommendations = Recommender::recommend(
			vec![
				RELATION_AB.clone(),
				RELATION_AC.clone(),
				RELATION_BA.clone(),
				RELATION_BC.clone(),
				RELATION_CA.clone(),
				RELATION_CB.clone(),
			]
			.into_iter()
			.map(Ok),
			1,
		)
		.unwrap()
		.collect::<BTreeSet<_>>();
		let expected_recommendations = bset! {
			Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_B.clone()]},
			Recommendation {post: POST_B.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_C.clone(), recommended_posts: vec![POST_A.clone()]},
		};
		assert_eq!(expected_recommendations, recommendations);
	}
}
