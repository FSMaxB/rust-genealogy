use crate::genealogy::relation::Relation;
use crate::helpers::collector::Collectors;
use crate::helpers::comparator::Comparator;
use crate::helpers::exception::Exception::{self, IllegalArgumentException};
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use crate::recommendation::Recommendation;
use crate::throw;

/// ```java
/// // Don't judge me for the name - recommend a better one (see what I did there?)
/// public class Recommender {
///
/// ```
pub struct Recommender;

impl Recommender {
	/// ```java
	/// public Stream<Recommendation> recommend(Stream<Relation> relations, int perPost) {
	/// 	if (perPost < 1)
	/// 		throw new IllegalArgumentException(
	/// 				"Number of recommendations per post must be greater zero: " + perPost);
	///
	/// 	Comparator<Relation> byPostThenByDecreasingScore =
	/// 			comparing((Relation relation) -> relation.post1().slug())
	/// 					.thenComparing(Relation::score)
	/// 					.reversed();
	/// 	Map<Post, List<Relation>> byPost = relations
	/// 			.sorted(byPostThenByDecreasingScore)
	/// 			.collect(groupingBy(Relation::post1));
	/// 	return byPost
	/// 			.entrySet().stream()
	/// 			.map(postWithRelations -> Recommendation.from(
	/// 					postWithRelations.getKey(),
	/// 					postWithRelations.getValue().stream().map(Relation::post2),
	/// 					perPost));
	///
	/// }
	/// ```
	pub fn recommend(relations: Stream<Relation>, per_post: i32) -> Result<Stream<Recommendation>, Exception> {
		if per_post < 1 {
			throw!(IllegalArgumentException(
				JString::from("Number of recommendations per post must be greater zero: ") + per_post
			));
		}

		let by_post_then_by_decreasing_score = Comparator::comparing(|relation: &Relation| relation.post1.slug())
			.then_comparing(Relation::score)
			.reversed();
		let by_post = relations
			.sorted(by_post_then_by_decreasing_score)?
			.collect(Collectors::grouping_by(Relation::post1))?;
		Ok(by_post.entry_set().stream().map(move |post_with_relations| {
			Recommendation::from(
				post_with_relations.get_key(),
				post_with_relations
					.get_value()
					.stream()
					.map(|relation| Ok(relation.post2())),
				per_post,
			)
		}))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::list::List;
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
			let post_a = PostTestHelper::create_with_slug("a".into())?;
			let post_b = PostTestHelper::create_with_slug("b".into())?;
			let post_c = PostTestHelper::create_with_slug("c".into())?;
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
			let recommendations =
				Recommender::recommend(Stream::of([self.relation_ac.clone()]), 1)?.collect(Collectors::to_set())?;
			let expected_recommendations =
				hset! {Recommendation {post: self.post_a.clone(), recommended_posts: List::of([self.post_c.clone()])}};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_one_post_two_relations(&self) -> Result<(), Exception> {
			let recommendations =
				Recommender::recommend(Stream::of([self.relation_ab.clone(), self.relation_ac.clone()]), 1)?
					.collect(Collectors::to_set())?;
			let expected_recommendations =
				hset! {Recommendation {post: self.post_a.clone(), recommended_posts: List::of([self.post_b.clone()])}};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_many_posts_one_relation_each(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(
				Stream::of([
					self.relation_ac.clone(),
					self.relation_bc.clone(),
					self.relation_cb.clone(),
				]),
				1,
			)?
			.collect(Collectors::to_set())?;
			let expected_recommendations = hset! {
				Recommendation {post: self.post_a.clone(), recommended_posts: List::of([self.post_c.clone()])},
				Recommendation {post: self.post_b.clone(), recommended_posts: List::of([self.post_c.clone()])},
				Recommendation {post: self.post_c.clone(), recommended_posts: List::of([self.post_b.clone()])},
			};
			assert_eq!(expected_recommendations, recommendations);
			Ok(())
		}

		fn for_many_posts_two_relations_each(&self) -> Result<(), Exception> {
			let recommendations = Recommender::recommend(
				Stream::of([
					self.relation_ab.clone(),
					self.relation_ac.clone(),
					self.relation_ba.clone(),
					self.relation_bc.clone(),
					self.relation_ca.clone(),
					self.relation_cb.clone(),
				]),
				1,
			)?
			.collect(Collectors::to_set())?;
			let expected_recommendations = hset! {
				Recommendation {post: self.post_a.clone(), recommended_posts: List::of([self.post_b.clone()])},
				Recommendation {post: self.post_b.clone(), recommended_posts: List::of([self.post_c.clone()])},
				Recommendation {post: self.post_c.clone(), recommended_posts: List::of([self.post_a.clone()])},
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
