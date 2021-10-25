use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogist::Genealogist;
use crate::genealogy::relation::Relation;
use crate::genealogy::weights::Weights;
use crate::helpers::collection::Collection;
use crate::helpers::exception::Exception;
use crate::helpers::list::ArrayList;
use crate::helpers::map::{JHashMap, Map};
use crate::helpers::stream::Stream;
use crate::post::Post;

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
		#[derive(Clone)]
		struct Posts {
			post1: Post,
			post2: Post,
		}

		impl Posts {
			fn new(post1: Post, post2: Post) -> Result<Self, Exception> {
				Ok(Self { post1, post2 })
			}
		}

		struct PostResearch {
			genealogist: Genealogist,
			posts: Posts,
		}

		impl PostResearch {
			fn new(genealogist: Genealogist, posts: Posts) -> Result<Self, Exception> {
				Ok(Self { genealogist, posts })
			}
		}

		self.posts.stream()
			.flat_map({
				let posts = self.posts.clone();
				move |post1| posts.stream().map(move |post2| Posts::new(post1.clone(), post2))
			})
			// no need to compare posts with themselves
			.filter(|posts| posts.post1 != posts.post2)
			.flat_map({
				let genealogists = self.genealogists.clone();
				move |posts| genealogists.stream().map({
					let posts = posts.clone();
					move |genealogist| PostResearch::new(genealogist, posts.clone())
				})
			})
			.map(|research| research.genealogist.infer(research.posts.post1, research.posts.post2))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::helpers::collector::Collectors;
	use crate::helpers::list::List;
	use crate::map_of;
	use crate::post::test::PostTestHelper;
	use literally::hset;
	use std::collections::HashSet;

	struct GenealogyTests {
		posts: Posts,
		tag_genealogist: Genealogist,
		link_genealogist: Genealogist,
		weights: Weights,
	}

	impl GenealogyTests {
		pub fn new() -> Result<Self, Exception> {
			let tag_relation = RelationType::new("tag".into())?;
			let link_relation = RelationType::new("link".into())?;
			let posts = Posts::new()?;
			Ok(Self {
				posts: posts.clone(),
				tag_genealogist: {
					let posts = posts.clone();
					let tag_relation = tag_relation.clone();
					move |post1: Post, post2: Post| {
						let score = posts.tag_score(&post1, &post2);
						TypedRelation::new(post1, post2, tag_relation.clone(), score)
					}
				}
				.into(),
				link_genealogist: {
					let posts = posts.clone();
					let link_relation = link_relation.clone();
					move |post1: Post, post2: Post| {
						let score = posts.link_score(&post1, &post2);
						TypedRelation::new(post1, post2, link_relation.clone(), score)
					}
				}
				.into(),
				weights: Weights::new(
					map_of!(tag_relation, posts.tag_weight, link_relation, posts.link_weight),
					0.5,
				),
			})
		}

		fn one_genealogist_two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone()]),
				List::of([self.tag_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?.collect(Collectors::to_set())?;
			let expected_relations = hset! {
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					self.posts.weighted_tag_score(&self.posts.a, &self.posts.b) as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					self.posts.weighted_tag_score(&self.posts.b, &self.posts.a) as i64,
				)?,
			};
			assert_eq!(expected_relations, relations);
			Ok(())
		}

		fn other_genealogist_two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone()]),
				List::of([self.link_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?.collect(Collectors::to_set())?;
			let expected_relations = hset! {
				Relation::new(
					self.posts.a.clone(),
					self.posts.b.clone(),
					self.posts.weighted_link_score(&self.posts.a, &self.posts.b) as i64,
				)?,
				Relation::new(
					self.posts.b.clone(),
					self.posts.a.clone(),
					self.posts.weighted_link_score(&self.posts.b, &self.posts.a) as i64,
				)?,
			};
			assert_eq!(expected_relations, relations);

			Ok(())
		}

		fn one_genealogist_three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()]),
				List::of([self.tag_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?.collect(Collectors::to_set())?;
			let expected_relations = vec![
				(self.posts.a.clone(), self.posts.b.clone()),
				(self.posts.a.clone(), self.posts.c.clone()),
				(self.posts.b.clone(), self.posts.a.clone()),
				(self.posts.b.clone(), self.posts.c.clone()),
				(self.posts.c.clone(), self.posts.a.clone()),
				(self.posts.c.clone(), self.posts.b.clone()),
			]
			.into_iter()
			.map(|(post1, post2)| {
				let score = self.posts.weighted_tag_score(&post1, &post2) as i64;
				Relation::new(post1, post2, score).unwrap()
			})
			.collect::<HashSet<_>>();

			assert_eq!(expected_relations, relations);
			Ok(())
		}

		fn two_genealogists_three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				List::of([self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()]),
				List::of([self.tag_genealogist.clone(), self.link_genealogist.clone()]),
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations()?.collect(Collectors::to_set())?;
			let expected_relations = vec![
				(self.posts.a.clone(), self.posts.b.clone()),
				(self.posts.a.clone(), self.posts.c.clone()),
				(self.posts.b.clone(), self.posts.a.clone()),
				(self.posts.b.clone(), self.posts.c.clone()),
				(self.posts.c.clone(), self.posts.a.clone()),
				(self.posts.c.clone(), self.posts.b.clone()),
			]
			.into_iter()
			.map(|(post1, post2)| {
				let score = self.posts.link_and_tag_score(&post1, &post2);
				Relation::new(post1, post2, score).unwrap()
			})
			.collect::<HashSet<_>>();

			assert_eq!(expected_relations, relations);
			Ok(())
		}
	}

	#[derive(Clone)]
	struct Posts {
		a: Post,
		b: Post,
		c: Post,
		tag_weight: f64,
		link_weight: f64,
	}

	impl Posts {
		pub fn new() -> Result<Posts, Exception> {
			Ok(Self {
				a: PostTestHelper::create_with_slug("a".into())?,
				b: PostTestHelper::create_with_slug("b".into())?,
				c: PostTestHelper::create_with_slug("c".into())?,
				tag_weight: 1.0,
				link_weight: 0.75,
			})
		}

		fn tag_score(&self, post1: &Post, post2: &Post) -> i64 {
			if post1 == post2 {
				100
			} else if (post1 == &self.a) && (post2 == &self.b) {
				80
			} else if (post1 == &self.a) && (post2 == &self.c) {
				60
			} else if (post1 == &self.b) && (post2 == &self.a) {
				70
			} else if (post1 == &self.b) && (post2 == &self.c) {
				50
			} else if (post1 == &self.c) && (post2 == &self.a) {
				50
			} else if (post1 == &self.c) && (post2 == &self.b) {
				40
			} else {
				0
			}
		}

		fn weighted_tag_score(&self, post1: &Post, post2: &Post) -> f64 {
			(self.tag_score(post1, post2) as f64) * self.tag_weight
		}

		fn link_score(&self, post1: &Post, post2: &Post) -> i64 {
			if post1 == post2 {
				100
			} else if (post1 == &self.a) && (post2 == &self.b) {
				60
			} else if (post1 == &self.a) && (post2 == &self.c) {
				40
			} else if (post1 == &self.b) && (post2 == &self.a) {
				50
			} else if (post1 == &self.b) && (post2 == &self.c) {
				30
			} else if (post1 == &self.c) && (post2 == &self.a) {
				30
			} else if (post1 == &self.c) && (post2 == &self.b) {
				20
			} else {
				0
			}
		}

		fn weighted_link_score(&self, post1: &Post, post2: &Post) -> f64 {
			(self.link_score(post1, post2) as f64) * self.link_weight
		}

		fn link_and_tag_score(&self, post1: &Post, post2: &Post) -> i64 {
			((self.weighted_tag_score(post1, post2) + self.weighted_link_score(post1, post2)) / 2.0) as i64
		}
	}

	#[test]
	fn one_genealogist_two_posts() {
		GenealogyTests::new().unwrap().one_genealogist_two_posts().unwrap();
	}

	#[ignore]
	#[test]
	fn other_genealogist_two_posts() {
		GenealogyTests::new().unwrap().other_genealogist_two_posts().unwrap();
	}

	#[test]
	fn one_genealogist_three_posts() {
		GenealogyTests::new().unwrap().one_genealogist_three_posts().unwrap();
	}

	#[ignore]
	#[test]
	fn two_genealogists_three_posts() {
		GenealogyTests::new().unwrap().two_genealogists_three_posts().unwrap();
	}
}
