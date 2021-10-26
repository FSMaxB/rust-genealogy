use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogist::Genealogist;
use crate::genealogy::relation::Relation;
use crate::genealogy::weights::Weights;
use crate::post::Post;
use genealogy_java_apis::collection::Collection;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::list::ArrayList;
use genealogy_java_apis::map::{JHashMap, Map};
use genealogy_java_apis::record;
use genealogy_java_apis::stream::Stream;

pub mod relation;
#[cfg(test)]
pub mod relation_test_helper;
pub mod weights;

/// ```java
/// public class Genealogy {
/// 	private final Collection<Post> posts;
/// 	private final Collection<Genealogist> genealogists;
/// 	private final Weights weights;
/// ```
pub struct Genealogy {
	posts: Collection<Post>,
	genealogists: Collection<Genealogist>,
	weights: Weights,
}

impl Genealogy {
	/// ```java
	/// public Genealogy(Collection<Post> posts, Collection<Genealogist> genealogists, Weights weights) {
	///		this.posts = requireNonNull(posts);
	///		this.genealogists = requireNonNull(genealogists);
	///		this.weights = requireNonNull(weights);
	///	}
	/// ```
	pub fn new(posts: Collection<Post>, genealogists: Collection<Genealogist>, weights: Weights) -> Self {
		Self {
			posts,
			genealogists,
			weights,
		}
	}

	/// ```java
	/// public Stream<Relation> inferRelations() {
	///		return aggregateTypedRelations(inferTypedRelations());
	///	}
	/// ```
	pub fn infer_relations(&self) -> Result<Stream<Relation>, Exception> {
		self.aggregate_typed_relations(self.infer_typed_relations())
	}

	/// ```java
	/// private Stream<Relation> aggregateTypedRelations(Stream<TypedRelation> typedRelations) {
	///		Map<Post, Map<Post, Collection<TypedRelation>>> sortedTypedRelations = new HashMap<>();
	///		typedRelations.forEach(relation -> sortedTypedRelations
	///				.computeIfAbsent(relation.post1(), __ -> new HashMap<>())
	///				.computeIfAbsent(relation.post2(), __ -> new ArrayList<>())
	///				.add(relation));
	///		return sortedTypedRelations
	///				.values().stream()
	///				.flatMap(postWithRelations -> postWithRelations.values().stream())
	///				.map(relations -> Relation.aggregate(relations.stream(), weights));
	///	}
	/// ```
	fn aggregate_typed_relations(&self, typed_relations: Stream<TypedRelation>) -> Result<Stream<Relation>, Exception> {
		let sorted_typed_relations: Map<Post, Map<Post, Collection<TypedRelation>>> = Map::new();
		typed_relations.for_each({
			let sorted_typed_relations = sorted_typed_relations.clone();
			move |relation| {
				sorted_typed_relations
					.clone()
					.compute_if_absent(relation.post1(), |_| JHashMap::new())
					.compute_if_absent(relation.post2(), |_| ArrayList::new())
					.add(relation);
				Ok(())
			}
		})?;

		Ok(sorted_typed_relations
			.values()
			.stream()
			.flat_map(|post_with_relations| post_with_relations.values().stream())
			.map({
				let weights = self.weights.clone();
				move |relations| Relation::aggregate(relations.stream(), weights.clone())
			}))
	}

	/// ```java
	/// private Stream<TypedRelation> inferTypedRelations() {
	///		record Posts(Post post1, Post post2) { }
	///		record PostResearch(Genealogist genealogist, Posts posts) { }
	///		return posts.stream()
	///				.flatMap(post1 -> posts.stream()
	///						.map(post2 -> new Posts(post1, post2)))
	///				// no need to compare posts with themselves
	///				.filter(posts -> posts.post1 != posts.post2)
	///				.flatMap(posts -> genealogists.stream()
	///						.map(genealogist -> new PostResearch(genealogist, posts)))
	///				.map(research -> research.genealogist()
	///						.infer(research.posts().post1(), research.posts().post2()));
	///	}
	/// ```
	fn infer_typed_relations(&self) -> Stream<TypedRelation> {
		#[record]
		#[derive(Clone)]
		struct Posts {
			post1: Post,
			post2: Post,
		}

		#[record]
		struct PostResearch {
			genealogist: Genealogist,
			posts: Posts,
		}

		self.posts.stream()
			.flat_map({
				let posts = self.posts.clone();
				move |post1| posts.stream().map(move |post2| Ok(Posts::new(post1.clone(), post2)))
			})
			// no need to compare posts with themselves
			.filter(|posts| posts.post1() != posts.post2())
			.flat_map({
				let genealogists = self.genealogists.clone();
				move |posts| genealogists.stream().map({
					move |genealogist| Ok(PostResearch::new(genealogist, posts.clone()))
				})
			})
			.map(|research| research.genealogist().infer(research.posts().post1(), research.posts().post2()))
	}
}

#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::post::test::PostTestHelper;
	use genealogy_java_apis::list::List;
	use genealogy_java_apis::map_of;
	use genealogy_java_apis::test::assert_that;

	/// ```java
	/// class GenealogyTests {
	/// ```
	struct GenealogyTests {
		posts: Posts,
		/// ```java
		/// private final RelationType tagRelation = new RelationType("tag");
		/// ```
		#[allow(unused)] // only used for constructing the other fields
		tag_relation: RelationType,
		/// ```java
		///	private final RelationType linkRelation = new RelationType("link");
		/// ```
		#[allow(unused)] // only used for constructing the other fields
		link_relation: RelationType,
		/// ```java
		///	private final Genealogist tagGenealogist = (Post1, Post2) ->
		///			new TypedRelation(Post1, Post2, tagRelation, tagScore(Post1, Post2));
		/// ```
		tag_genealogist: Genealogist,
		/// ```java
		///	private final Genealogist linkGenealogist = (Post1, Post2) ->
		///			new TypedRelation(Post1, Post2, linkRelation, linkScore(Post1, Post2));
		/// ```
		link_genealogist: Genealogist,
		/// ```java
		///	private final Weights weights = new Weights(
		///			Map.of(
		///					tagRelation, TAG_WEIGHT,
		///					linkRelation, LINK_WEIGHT),
		///			0.5);
		/// ```
		weights: Weights,
	}

	impl GenealogyTests {
		/// ```java
		/// private static final int TAG_SCORE_A_B = 80;
		/// ```
		const TAG_SCORE_A_B: i32 = 80;
		/// ```java
		///	private static final int TAG_SCORE_A_C = 60;
		/// ```
		const TAG_SCORE_A_C: i32 = 60;
		/// ```java
		///	private static final int TAG_SCORE_B_A = 70;
		/// ```
		const TAG_SCORE_B_A: i32 = 70;
		/// ```java
		///	private static final int TAG_SCORE_B_C = 50;
		/// ```
		const TAG_SCORE_B_C: i32 = 50;
		/// ```java
		///	private static final int TAG_SCORE_C_A = 50;
		/// ```
		const TAG_SCORE_C_A: i32 = 50;
		/// ```java
		///	private static final int TAG_SCORE_C_B = 40;
		/// ```
		const TAG_SCORE_C_B: i32 = 40;

		/// ```java
		///	private static final int LINK_SCORE_A_B = 60;
		/// ```
		const LINK_SCORE_A_B: i32 = 60;
		/// ```java
		///	private static final int LINK_SCORE_A_C = 40;
		/// ```
		const LINK_SCORE_A_C: i32 = 40;
		/// ```java
		///	private static final int LINK_SCORE_B_A = 50;
		/// ```
		const LINK_SCORE_B_A: i32 = 50;
		/// ```java
		///	private static final int LINK_SCORE_B_C = 30;
		/// ```
		const LINK_SCORE_B_C: i32 = 30;
		/// ```java
		///	private static final int LINK_SCORE_C_A = 30;
		/// ```
		const LINK_SCORE_C_A: i32 = 30;
		/// ```java
		///	private static final int LINK_SCORE_C_B = 20;
		/// ```
		const LINK_SCORE_C_B: i32 = 20;

		/// ```java
		///	private static final double TAG_WEIGHT = 1.0;
		/// ```
		const TAG_WEIGHT: f64 = 1.0;
		/// ```java
		///	private static final double LINK_WEIGHT = 0.75;
		/// ```
		const LINK_WEIGHT: f64 = 0.75;

		/// ```java
		/// private final Post postA = PostTestHelper.createWithSlug("a");
		///	private final Post postB = PostTestHelper.createWithSlug("b");
		///	private final Post postC = PostTestHelper.createWithSlug("c");
		///
		///	private final RelationType tagRelation = new RelationType("tag");
		///	private final RelationType linkRelation = new RelationType("link");
		///
		///	private final Genealogist tagGenealogist = (Post1, Post2) ->
		///			new TypedRelation(Post1, Post2, tagRelation, tagScore(Post1, Post2));
		///	private final Genealogist linkGenealogist = (Post1, Post2) ->
		///			new TypedRelation(Post1, Post2, linkRelation, linkScore(Post1, Post2));
		///
		///	private final Weights weights = new Weights(
		///			Map.of(
		///					tagRelation, TAG_WEIGHT,
		///					linkRelation, LINK_WEIGHT),
		///			0.5);
		/// ```
		pub fn new() -> Result<Self, Exception> {
			let tag_relation = RelationType::new("tag".into())?;
			let link_relation = RelationType::new("link".into())?;
			let posts = Posts::new()?;
			Ok(Self {
				posts: posts.clone(),
				tag_relation: tag_relation.clone(),
				link_relation: link_relation.clone(),
				tag_genealogist: {
					let posts = posts.clone();
					let tag_relation = tag_relation.clone();
					move |post1: Post, post2: Post| {
						TypedRelation::new(
							post1.clone(),
							post2.clone(),
							tag_relation.clone(),
							posts.tag_score(post1, post2) as i64,
						)
					}
				}
				.into(),
				link_genealogist: {
					let posts = posts;
					let link_relation = link_relation.clone();
					move |post1: Post, post2: Post| {
						TypedRelation::new(
							post1.clone(),
							post2.clone(),
							link_relation.clone(),
							posts.link_score(post1, post2) as i64,
						)
					}
				}
				.into(),
				weights: Weights::new(
					map_of!(tag_relation, Self::TAG_WEIGHT, link_relation, Self::LINK_WEIGHT,),
					0.5,
				),
			})
		}

		/// ```java
		/// @Test
		///	void oneGenealogist_twoPosts() {
		///		var genealogy = new Genealogy(
		///				List.of(postA, postB),
		///				List.of(tagGenealogist),
		///				weights);
		///
		///		var relations = genealogy.inferRelations();
		///
		///		assertThat(relations).containsExactlyInAnyOrder(
		///				new Relation(postA, postB, round(TAG_SCORE_A_B * TAG_WEIGHT)),
		///				new Relation(postB, postA, round(TAG_SCORE_B_A * TAG_WEIGHT))
		///		);
		///	}
		/// ```
		fn one_genealogist__two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone()]),
				List::of([self.tag_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?;

			assert_that(relations).contains_exactly_in_any_order([
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					((Self::TAG_SCORE_A_B as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					((Self::TAG_SCORE_B_A as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
			]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void otherGenealogist_twoPosts() {
		///		var genealogy = new Genealogy(
		///				List.of(postA, postB),
		///				List.of(linkGenealogist),
		///				weights);
		///
		///		var relations = genealogy.inferRelations();
		///
		///		assertThat(relations).containsExactlyInAnyOrder(
		///				new Relation(postA, postB, round(LINK_SCORE_A_B * LINK_WEIGHT)),
		///				new Relation(postB, postA, round(LINK_SCORE_B_A * LINK_WEIGHT))
		///		);
		///	}
		/// ```
		fn other_genealogist__two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone()]),
				List::of([self.link_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?;

			assert_that(relations).contains_exactly_in_any_order([
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					((Self::LINK_SCORE_A_B as f64) * Self::LINK_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					((Self::LINK_SCORE_B_A as f64) * Self::LINK_WEIGHT).round() as i64,
				)?,
			]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void oneGenealogist_threePosts() {
		///		var genealogy = new Genealogy(
		///				List.of(postA, postB, postC),
		///				List.of(tagGenealogist),
		///				weights);
		///
		///		var relations = genealogy.inferRelations();
		///
		///		assertThat(relations).containsExactlyInAnyOrder(
		///				new Relation(postA, postB, round(TAG_SCORE_A_B * TAG_WEIGHT)),
		///				new Relation(postA, postC, round(TAG_SCORE_A_C * TAG_WEIGHT)),
		///				new Relation(postB, postA, round(TAG_SCORE_B_A * TAG_WEIGHT)),
		///				new Relation(postB, postC, round(TAG_SCORE_B_C * TAG_WEIGHT)),
		///				new Relation(postC, postA, round(TAG_SCORE_C_A * TAG_WEIGHT)),
		///				new Relation(postC, postB, round(TAG_SCORE_C_B * TAG_WEIGHT))
		///		);
		///	}
		/// ```
		fn one_genealogist__three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()]),
				List::of([self.tag_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?;

			assert_that(relations).contains_exactly_in_any_order([
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					((Self::TAG_SCORE_A_B as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.a.clone(),
					self.posts.c.clone(),
					((Self::TAG_SCORE_A_C as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					((Self::TAG_SCORE_B_A as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.c.clone(),
					((Self::TAG_SCORE_B_C as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.c.clone(),
					self.posts.a.clone(),
					((Self::TAG_SCORE_C_A as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
				Relation::new(
					self.posts.c.clone(),
					self.posts.b.clone(),
					((Self::TAG_SCORE_C_B as f64) * Self::TAG_WEIGHT).round() as i64,
				)?,
			]);
			Ok(())
		}

		/// ```java
		/// @Test
		///	void twoGenealogists_threePosts() {
		///		var genealogy = new Genealogy(
		///				List.of(postA, postB, postC),
		///				List.of(tagGenealogist, linkGenealogist),
		///				weights);
		///
		///		var relations = genealogy.inferRelations();
		///
		///		assertThat(relations).containsExactlyInAnyOrder(
		///				new Relation(postA, postB, round((TAG_SCORE_A_B * TAG_WEIGHT + LINK_SCORE_A_B * LINK_WEIGHT) / 2)),
		///				new Relation(postA, postC, round((TAG_SCORE_A_C * TAG_WEIGHT + LINK_SCORE_A_C * LINK_WEIGHT) / 2)),
		///				new Relation(postB, postA, round((TAG_SCORE_B_A * TAG_WEIGHT + LINK_SCORE_B_A * LINK_WEIGHT) / 2)),
		///				new Relation(postB, postC, round((TAG_SCORE_B_C * TAG_WEIGHT + LINK_SCORE_B_C * LINK_WEIGHT) / 2)),
		///				new Relation(postC, postA, round((TAG_SCORE_C_A * TAG_WEIGHT + LINK_SCORE_C_A * LINK_WEIGHT) / 2)),
		///				new Relation(postC, postB, round((TAG_SCORE_C_B * TAG_WEIGHT + LINK_SCORE_C_B * LINK_WEIGHT) / 2))
		///		);
		///	}
		/// ```
		fn two_genealogists__three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()]),
				List::of([self.tag_genealogist.clone(), self.link_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?;

			assert_that(relations).contains_exactly_in_any_order([
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					(((Self::TAG_SCORE_A_B as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_A_B as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
				Relation::new(
					self.posts.a.clone(),
					self.posts.c.clone(),
					(((Self::TAG_SCORE_A_C as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_A_C as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					(((Self::TAG_SCORE_B_A as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_B_A as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.c.clone(),
					(((Self::TAG_SCORE_B_C as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_B_C as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
				Relation::new(
					self.posts.c.clone(),
					self.posts.a.clone(),
					(((Self::TAG_SCORE_C_A as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_C_A as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
				Relation::new(
					self.posts.c.clone(),
					self.posts.b.clone(),
					(((Self::TAG_SCORE_C_B as f64) * Self::TAG_WEIGHT
						+ (Self::LINK_SCORE_C_B as f64) * Self::LINK_WEIGHT)
						/ 2.0)
						.round() as i64,
				)?,
			]);
			Ok(())
		}
	}

	/// tagScore and linkScore access the posts, but are also called when
	/// initialising the tagGenealogist and linkGenealogist, meaning the posts
	/// need to be fully constructed before those are initialized, therefore
	/// we need a separate subtype for the posts
	#[derive(Clone)]
	struct Posts {
		a: Post,
		b: Post,
		c: Post,
	}

	impl Posts {
		/// ```java
		/// private final Post postA = PostTestHelper.createWithSlug("a");
		///	private final Post postB = PostTestHelper.createWithSlug("b");
		///	private final Post postC = PostTestHelper.createWithSlug("c");
		/// ```
		fn new() -> Result<Self, Exception> {
			Ok(Self {
				a: PostTestHelper::create_with_slug("a".into())?,
				b: PostTestHelper::create_with_slug("b".into())?,
				c: PostTestHelper::create_with_slug("c".into())?,
			})
		}

		/// ```java
		/// private int tagScore(Post post1, Post post2) {
		///    	if (post1 == post2)
		///    		return 100;
		///    	if (post1 == postA && post2 == postB)
		///    		return TAG_SCORE_A_B;
		///    	if (post1 == postA && post2 == postC)
		///    		return TAG_SCORE_A_C;
		///    	if (post1 == postB && post2 == postA)
		///    		return TAG_SCORE_B_A;
		///    	if (post1 == postB && post2 == postC)
		///    		return TAG_SCORE_B_C;
		///    	if (post1 == postC && post2 == postA)
		///    		return TAG_SCORE_C_A;
		///    	if (post1 == postC && post2 == postB)
		///    		return TAG_SCORE_C_B;
		///    	return 0;
		///    }
		/// ```
		fn tag_score(&self, post1: Post, post2: Post) -> i32 {
			if post1 == post2 {
				100
			} else if post1 == self.a && post2 == self.b {
				GenealogyTests::TAG_SCORE_A_B
			} else if post1 == self.a && post2 == self.c {
				GenealogyTests::TAG_SCORE_A_C
			} else if post1 == self.b && post2 == self.a {
				GenealogyTests::TAG_SCORE_B_A
			} else if post1 == self.b && post2 == self.c {
				GenealogyTests::TAG_SCORE_B_C
			} else if post1 == self.c && post2 == self.a {
				GenealogyTests::TAG_SCORE_C_A
			} else if post1 == self.c && post2 == self.b {
				GenealogyTests::TAG_SCORE_C_B
			} else {
				0
			}
		}

		/// ```java
		/// private int linkScore(Post post1, Post post2) {
		///    	if (post1 == post2)
		///    		return 100;
		///    	if (post1 == postA && post2 == postB)
		///    		return LINK_SCORE_A_B;
		///    	if (post1 == postA && post2 == postC)
		///    		return LINK_SCORE_A_C;
		///    	if (post1 == postB && post2 == postA)
		///    		return LINK_SCORE_B_A;
		///    	if (post1 == postB && post2 == postC)
		///    		return LINK_SCORE_B_C;
		///    	if (post1 == postC && post2 == postA)
		///    		return LINK_SCORE_C_A;
		///    	if (post1 == postC && post2 == postB)
		///    		return LINK_SCORE_C_B;
		///    	return 0;
		///    }
		/// ```
		fn link_score(&self, post1: Post, post2: Post) -> i32 {
			if post1 == post2 {
				100
			} else if post1 == self.a && post2 == self.b {
				GenealogyTests::LINK_SCORE_A_B
			} else if post1 == self.a && post2 == self.c {
				GenealogyTests::LINK_SCORE_A_C
			} else if post1 == self.b && post2 == self.a {
				GenealogyTests::LINK_SCORE_B_A
			} else if post1 == self.b && post2 == self.c {
				GenealogyTests::LINK_SCORE_B_C
			} else if post1 == self.c && post2 == self.a {
				GenealogyTests::LINK_SCORE_C_A
			} else if post1 == self.c && post2 == self.b {
				GenealogyTests::LINK_SCORE_C_B
			} else {
				0
			}
		}
	}

	#[test]
	fn one_genealogist__two_posts() {
		GenealogyTests::new().unwrap().one_genealogist__two_posts().unwrap();
	}

	#[test]
	fn other_genealogist__two_posts() {
		GenealogyTests::new().unwrap().other_genealogist__two_posts().unwrap();
	}

	#[test]
	fn one_genealogist__three_posts() {
		GenealogyTests::new().unwrap().one_genealogist__three_posts().unwrap();
	}

	#[test]
	fn two_genealogists__three_posts() {
		GenealogyTests::new().unwrap().two_genealogists__three_posts().unwrap();
	}
}
