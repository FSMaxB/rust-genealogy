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
/// ```
pub struct Recommender;

impl Recommender {
	/// ```java
	/// public class Recommender {
	/// ```
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self
	}

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
	pub fn recommend(&self, relations: Stream<Relation>, per_post: i32) -> Result<Stream<Recommendation>, Exception> {
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

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogy::relation_test_helper::RelationTestHelper;
	use crate::helpers::list::List;
	use crate::helpers::test::assert_that;
	use crate::post::test::PostTestHelper;
	use crate::post::Post;

	/// ```java
	/// class RecommenderTests {
	/// 	private final Post postA = PostTestHelper.createWithSlug("a");
	///		private final Post postB = PostTestHelper.createWithSlug("b");
	///		private final Post postC = PostTestHelper.createWithSlug("c");
	///
	///		private final Relation relation_AB = RelationTestHelper.create(postA, postB, 60L);
	///		private final Relation relation_AC = RelationTestHelper.create(postA, postC, 40L);
	///		private final Relation relation_BA = RelationTestHelper.create(postB, postA, 50L);
	///		private final Relation relation_BC = RelationTestHelper.create(postB, postC, 70L);
	///		private final Relation relation_CA = RelationTestHelper.create(postC, postA, 80L);
	///		private final Relation relation_CB = RelationTestHelper.create(postC, postB, 60L);
	///
	///		private final Recommender recommender = new Recommender();
	/// ```
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
		recommender: Recommender,
	}

	impl RecommenderTests {
		/// ```java
		/// private final Post postA = PostTestHelper.createWithSlug("a");
		///	private final Post postB = PostTestHelper.createWithSlug("b");
		///	private final Post postC = PostTestHelper.createWithSlug("c");
		///
		///	private final Relation relation_AB = RelationTestHelper.create(postA, postB, 60L);
		///	private final Relation relation_AC = RelationTestHelper.create(postA, postC, 40L);
		///	private final Relation relation_BA = RelationTestHelper.create(postB, postA, 50L);
		///	private final Relation relation_BC = RelationTestHelper.create(postB, postC, 70L);
		///	private final Relation relation_CA = RelationTestHelper.create(postC, postA, 80L);
		///	private final Relation relation_CB = RelationTestHelper.create(postC, postB, 60L);
		///
		///	private final Recommender recommender = new Recommender();
		/// ```
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
				recommender: Recommender::new(),
			})
		}

		/// ```java
		/// @Test
		///	void forOnePost_oneRelation() {
		///		var recommendations = recommender.recommend(
		///				Stream.of(relation_AC),
		///				1);
		///
		///		assertThat(recommendations).containsExactlyInAnyOrder(
		///				new Recommendation(postA, List.of(postC)));
		///	}
		/// ```
		fn for_one_post__one_relation(&self) -> Result<(), Exception> {
			let recommendations = self.recommender.recommend(Stream::of([self.relation_ac.clone()]), 1)?;

			assert_that(recommendations).contains_exactly_in_any_order([Recommendation::new(
				self.post_a.clone(),
				List::of([self.post_c.clone()]),
			)]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void forOnePost_twoRelations() {
		///		var recommendations = recommender.recommend(
		///				Stream.of(relation_AB, relation_AC),
		///				1);
		///
		///		assertThat(recommendations).containsExactlyInAnyOrder(
		///				new Recommendation(postA, List.of(postB)));
		///	}
		/// ```
		fn for_one_post__two_relations(&self) -> Result<(), Exception> {
			let recommendations = self
				.recommender
				.recommend(Stream::of([self.relation_ab.clone(), self.relation_ac.clone()]), 1)?;

			assert_that(recommendations).contains_exactly_in_any_order([Recommendation::new(
				self.post_a.clone(),
				List::of([self.post_b.clone()]),
			)]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void forManyPosts_oneRelationEach() {
		///		var recommendations = recommender.recommend(
		///				Stream.of(relation_AC, relation_BC, relation_CB),
		///				1);
		///
		///		assertThat(recommendations).containsExactlyInAnyOrder(
		///				new Recommendation(postA, List.of(postC)),
		///				new Recommendation(postB, List.of(postC)),
		///				new Recommendation(postC, List.of(postB))
		///		);
		///	}
		/// ```
		fn for_many_posts__one_relation_each(&self) -> Result<(), Exception> {
			let recommendations = self.recommender.recommend(
				Stream::of([
					self.relation_ac.clone(),
					self.relation_bc.clone(),
					self.relation_cb.clone(),
				]),
				1,
			)?;

			assert_that(recommendations).contains_exactly_in_any_order([
				Recommendation::new(self.post_a.clone(), List::of([self.post_c.clone()])),
				Recommendation::new(self.post_b.clone(), List::of([self.post_c.clone()])),
				Recommendation::new(self.post_c.clone(), List::of([self.post_b.clone()])),
			]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void forManyPosts_twoRelationsEach() {
		///		var recommendations = recommender.recommend(
		///				Stream.of(relation_AB, relation_AC, relation_BA, relation_BC, relation_CA, relation_CB),
		///				1);
		///
		///		assertThat(recommendations).containsExactlyInAnyOrder(
		///				new Recommendation(postA, List.of(postB)),
		///				new Recommendation(postB, List.of(postC)),
		///				new Recommendation(postC, List.of(postA))
		///		);
		///	}
		/// ```
		fn for_many_posts__two_relations_each(&self) -> Result<(), Exception> {
			let recommendations = self.recommender.recommend(
				Stream::of([
					self.relation_ab.clone(),
					self.relation_ac.clone(),
					self.relation_ba.clone(),
					self.relation_bc.clone(),
					self.relation_ca.clone(),
					self.relation_cb.clone(),
				]),
				1,
			)?;

			assert_that(recommendations).contains_exactly_in_any_order([
				Recommendation {
					post: self.post_a.clone(),
					recommended_posts: List::of([self.post_b.clone()]),
				},
				Recommendation {
					post: self.post_b.clone(),
					recommended_posts: List::of([self.post_c.clone()]),
				},
				Recommendation {
					post: self.post_c.clone(),
					recommended_posts: List::of([self.post_a.clone()]),
				},
			]);
			Ok(())
		}
	}

	#[test]
	fn for_one_post__one_relation() {
		RecommenderTests::new().unwrap().for_one_post__one_relation().unwrap();
	}

	#[test]
	fn for_one_post__two_relations() {
		RecommenderTests::new().unwrap().for_one_post__two_relations().unwrap();
	}

	#[test]
	fn for_many_posts__one_relation_each() {
		RecommenderTests::new()
			.unwrap()
			.for_many_posts__one_relation_each()
			.unwrap();
	}

	#[test]
	fn for_many_posts__two_relations_each() {
		RecommenderTests::new()
			.unwrap()
			.for_many_posts__two_relations_each()
			.unwrap();
	}
}
