use crate::genealogy::relation::Relation;
use crate::helpers::exception::Exception;
use crate::recommendation::Recommendation;
use resiter::Map;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::num::NonZeroUsize;
use std::ops::Deref;

pub struct Recommender;

impl Recommender {
	pub fn recommend(
		relations: impl Iterator<Item = Result<Relation, Exception>>,
		per_post: NonZeroUsize,
	) -> Result<impl Iterator<Item = Recommendation>, Exception> {
		let by_post = relations
			.map_ok(RelationSortedByPostThenByDecreasingScore::from)
			.map_ok(|relation| (relation.post1.clone(), relation))
			.try_fold(HashMap::new(), |mut map, result| {
				let (post1, relation) = result?;
				// NOTE: The HashSet already puts the relations into a sorted order
				map.entry(post1).or_insert_with(HashSet::new).insert(relation);
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

#[derive(PartialEq, Eq, Hash)]
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
	use crate::genealogy::score::Score;
	use crate::post::test::PostTestHelper;
	use crate::post::Post;
	use lazy_static::lazy_static;
	use literally::hset;
	use std::sync::Arc;

	lazy_static! {
		static ref POST_A: Arc<Post> = Arc::new(PostTestHelper::create_with_slug("a").unwrap());
		static ref POST_B: Arc<Post> = Arc::new(PostTestHelper::create_with_slug("b").unwrap());
		static ref POST_C: Arc<Post> = Arc::new(PostTestHelper::create_with_slug("c").unwrap());
		static ref RELATION_AB: Relation =
			RelationTestHelper::create(POST_A.clone(), POST_B.clone(), 60.try_into().unwrap());
		static ref RELATION_AC: Relation =
			RelationTestHelper::create(POST_A.clone(), POST_C.clone(), 40.try_into().unwrap());
		static ref RELATION_BA: Relation =
			RelationTestHelper::create(POST_B.clone(), POST_A.clone(), 50.try_into().unwrap());
		static ref RELATION_BC: Relation =
			RelationTestHelper::create(POST_B.clone(), POST_C.clone(), 70.try_into().unwrap());
		static ref RELATION_CA: Relation =
			RelationTestHelper::create(POST_C.clone(), POST_A.clone(), 80.try_into().unwrap());
		static ref RELATION_CB: Relation =
			RelationTestHelper::create(POST_C.clone(), POST_B.clone(), 60.try_into().unwrap());
	}

	struct RelationTestHelper;

	impl RelationTestHelper {
		// WTF: Why a static method just to call the constructor with the exact same arguments. WTF x2
		fn create(post1: Arc<Post>, post2: Arc<Post>, score: Score) -> Relation {
			Relation { post1, post2, score }
		}
	}

	#[test]
	fn for_one_post_one_relation() {
		let recommendations = Recommender::recommend(
			vec![RELATION_AC.clone()].into_iter().map(Ok),
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap()
		.collect::<HashSet<_>>();
		let expected_recommendations =
			hset! {Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_C.clone()]}};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[ignore]
	#[test]
	fn for_one_post_two_relations() {
		let recommendations = Recommender::recommend(
			vec![RELATION_AB.clone(), RELATION_AC.clone()].into_iter().map(Ok),
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap()
		.collect::<HashSet<_>>();
		let expected_recommendations =
			hset! {Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_B.clone()]}};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[test]
	fn for_many_posts_one_relation_each() {
		let recommendations = Recommender::recommend(
			vec![RELATION_AC.clone(), RELATION_BC.clone(), RELATION_CB.clone()]
				.into_iter()
				.map(Ok),
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap()
		.collect::<HashSet<_>>();
		let expected_recommendations = hset! {
			Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_B.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_C.clone(), recommended_posts: vec![POST_B.clone()]},
		};
		assert_eq!(expected_recommendations, recommendations);
	}

	#[ignore]
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
			NonZeroUsize::new(1).unwrap(),
		)
		.unwrap()
		.collect::<HashSet<_>>();
		let expected_recommendations = hset! {
			Recommendation {post: POST_A.clone(), recommended_posts: vec![POST_B.clone()]},
			Recommendation {post: POST_B.clone(), recommended_posts: vec![POST_C.clone()]},
			Recommendation {post: POST_C.clone(), recommended_posts: vec![POST_A.clone()]},
		};
		assert_eq!(expected_recommendations, recommendations);
	}
}
