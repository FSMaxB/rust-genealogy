use crate::genealogy::relation::Relation;
use crate::helpers::exception::Exception;
use crate::recommendation::Recommendation;
use resiter::Map;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

pub struct Recommender;

impl Recommender {
	pub fn recommend(
		relations: impl Iterator<Item = Result<Relation, Exception>>,
		per_post: usize,
	) -> Result<impl Iterator<Item = Result<Recommendation, Exception>>, Exception> {
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
			Recommendation::from(
				post,
				sorted_relations
					.into_iter()
					.map(|relation| Ok::<_, Exception>(relation.post2.clone()))
					.into(),
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
	use crate::post::test::PostTestHelper;
	use crate::post::Post;
	use literally::hset;

	struct RecommenderTests {
		post_a: Post,
		post_b: Post,
		post_c: Post,
		relation_ab: Relation,
		relation_ac: Relation,
		relation_ba: Relation,
		relation_bc: Relation,
		relation_ca: Relation,
		relation_cb: Relation,
	}

	impl RecommenderTests {
		fn new() -> Result<Self, Exception> {
			let post_a = PostTestHelper::create_with_slug("a")?;
			let post_b = PostTestHelper::create_with_slug("b")?;
			let post_c = PostTestHelper::create_with_slug("c")?;
			Ok(Self {
				post_a: post_a.clone(),
				post_b: post_b.clone(),
				post_c: post_c.clone(),
				relation_ab: RelationTestHelper::create(post_a.clone(), post_b.clone(), 60)?,
				relation_ac: RelationTestHelper::create(post_a.clone(), post_c.clone(), 40)?,
				relation_ba: RelationTestHelper::create(post_b.clone(), post_a.clone(), 50)?,
				relation_bc: RelationTestHelper::create(post_b.clone(), post_c.clone(), 70)?,
				relation_ca: RelationTestHelper::create(post_c.clone(), post_a, 80)?,
				relation_cb: RelationTestHelper::create(post_c, post_b, 60)?,
			})
		}

		fn for_one_post_one_relation(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(vec![self.relation_ac.clone()].into_iter().map(Ok), 1)?
				.collect::<Result<HashSet<_>, Exception>>()?;
			let expected_recommendations =
				hset! {Recommendation {post: self.post_a.clone(), recommended_posts: vec![self.post_c.clone()]}};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_one_post_two_relations(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(
				vec![self.relation_ab.clone(), self.relation_ac.clone()]
					.into_iter()
					.map(Ok),
				1,
			)?
			.collect::<Result<HashSet<_>, Exception>>()?;
			let expected_recommendations =
				hset! {Recommendation {post: self.post_a.clone(), recommended_posts: vec![self.post_b.clone()]}};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_many_posts_one_relation_each(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(
				vec![
					self.relation_ac.clone(),
					self.relation_bc.clone(),
					self.relation_cb.clone(),
				]
				.into_iter()
				.map(Ok),
				1,
			)?
			.collect::<Result<HashSet<_>, Exception>>()?;
			let expected_recommendations = hset! {
				Recommendation {post: self.post_a.clone(), recommended_posts: vec![self.post_c.clone()]},
				Recommendation {post: self.post_b.clone(), recommended_posts: vec![self.post_c.clone()]},
				Recommendation {post: self.post_c.clone(), recommended_posts: vec![self.post_b.clone()]},
			};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_many_posts_two_relations_each(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(
				vec![
					self.relation_ab.clone(),
					self.relation_ac.clone(),
					self.relation_ba.clone(),
					self.relation_bc.clone(),
					self.relation_ca.clone(),
					self.relation_cb.clone(),
				]
				.into_iter()
				.map(Ok),
				1,
			)?
			.collect::<Result<HashSet<_>, Exception>>()?;
			let expected_recommendations = hset! {
				Recommendation {post: self.post_a.clone(), recommended_posts: vec![self.post_b.clone()]},
				Recommendation {post: self.post_b.clone(), recommended_posts: vec![self.post_c.clone()]},
				Recommendation {post: self.post_c.clone(), recommended_posts: vec![self.post_a.clone()]},
			};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}
	}

	struct RelationTestHelper;

	impl RelationTestHelper {
		// WTF: Why a static method just to call the constructor with the exact same arguments. WTF x2
		fn create(post1: Post, post2: Post, score: i64) -> Result<Relation, Exception> {
			Relation::new(post1, post2, score)
		}
	}

	#[test]
	fn for_one_post_one_relation() {
		RecommenderTests::new().unwrap().for_one_post_one_relation().unwrap();
	}

	#[ignore]
	#[test]
	fn for_one_post_two_relations() {
		RecommenderTests::new().unwrap().for_one_post_two_relations().unwrap();
	}

	#[test]
	fn for_many_posts_one_relation_each() {
		RecommenderTests::new()
			.unwrap()
			.for_many_posts_one_relation_each()
			.unwrap();
	}

	#[ignore]
	#[test]
	fn for_many_posts_two_relations_each() {
		RecommenderTests::new()
			.unwrap()
			.for_many_posts_two_relations_each()
			.unwrap();
	}
}
