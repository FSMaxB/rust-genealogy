use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogist::Genealogist;
use crate::genealogy::relation::Relation;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::iterator::result_iterator::ResultIteratorExtension;
use crate::post::Post;
use resiter::Map;
use std::collections::HashMap;
use std::rc::Rc;

pub mod relation;
pub mod score;
pub mod weight;
pub mod weights;

pub struct Genealogy {
	posts: Vec<Rc<Post>>,
	genealogists: Vec<Rc<dyn Genealogist>>,
	weights: Rc<Weights>,
}

impl Genealogy {
	pub fn new(posts: Vec<Rc<Post>>, genealogists: Vec<Rc<dyn Genealogist>>, weights: Rc<Weights>) -> Self {
		Self {
			posts,
			genealogists,
			weights,
		}
	}

	pub fn infer_relations(&self) -> impl Iterator<Item = Result<Relation, Exception>> {
		self.aggregate_typed_relations(infer_typed_relations(self.posts.clone(), self.genealogists.clone()))
	}

	fn aggregate_typed_relations(
		&self,
		mut typed_relations: impl Iterator<Item = Result<TypedRelation, Exception>>,
	) -> impl Iterator<Item = Result<Relation, Exception>> {
		let sorted_typed_relations = typed_relations.try_fold(HashMap::new(), |mut map, result| {
			let relation = result?;
			map.entry(relation.post1.clone())
				.or_insert_with(HashMap::new)
				.entry(relation.post2.clone())
				.or_insert_with(Vec::new)
				.push(relation);
			Ok(map)
		});
		let weights = self.weights.clone();
		sorted_typed_relations
			.into_result_iterator()
			.map_ok(|(_, value)| value)
			.flat_map(|post_with_relations| post_with_relations.into_result_iterator().map_ok(|(_, value)| value))
			.map_ok(move |relations| Relation::aggregate(relations.iter(), &weights))
			.map(|result| result.and_then(std::convert::identity))
	}
}

fn infer_typed_relations(
	posts: Vec<Rc<Post>>,
	genealogists: Vec<Rc<dyn Genealogist>>,
) -> impl Iterator<Item = Result<TypedRelation, Exception>> {
	// FIXME: I have to clone quite a lot here. It's just references, but still pretty horrible.
	posts
		.clone()
		.into_iter()
		.flat_map(move |post1| posts.clone().into_iter().map(move |post2| (post1.clone(), post2)))
		.filter(|(post1, post2)| post1 != post2)
		.flat_map(move |posts| {
			genealogists
				.clone()
				.into_iter()
				.map(move |genealogist| (genealogist, posts.clone()))
		})
		.map(|(genealogist, (post1, post2))| genealogist.infer(post1, post2))
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::genealogist::relation_type::RelationType;
	use crate::genealogy::score::WeightedScore;
	use crate::genealogy::weight::{weight, Weight};
	use crate::post::test::PostTestHelper;
	use literally::{hmap, hset};
	use std::collections::HashSet;
	use std::rc::Rc;

	struct GenealogyTests {
		posts: Posts,
		tag_genealogist: Rc<dyn Genealogist>,
		link_genealogist: Rc<dyn Genealogist>,
		weights: Rc<Weights>,
	}

	impl GenealogyTests {
		pub fn new() -> Result<Self, Exception> {
			let tag_relation = RelationType::new("tag".to_string())?;
			let link_relation = RelationType::new("link".to_string())?;
			let posts = Posts::new()?;
			Ok(Self {
				posts: posts.clone(),
				tag_genealogist: Rc::new({
					let posts = posts.clone();
					let tag_relation = tag_relation.clone();
					move |post1: Rc<Post>, post2: Rc<Post>| {
						let score = posts.tag_score(&post1, &post2);
						Ok(TypedRelation::new(post1, post2, tag_relation.clone(), score)?)
					}
				}),
				link_genealogist: Rc::new({
					let posts = posts.clone();
					let link_relation = link_relation.clone();
					move |post1: Rc<Post>, post2: Rc<Post>| {
						let score = posts.link_score(&post1, &post2);
						Ok(TypedRelation::new(post1, post2, link_relation.clone(), score)?)
					}
				}),
				weights: Rc::new(Weights::new(
					hmap! {
						tag_relation.clone() => posts.tag_weight,
						link_relation.clone() => posts.link_weight,
					},
					weight(0.5),
				)),
			})
		}

		fn one_genealogist_two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				vec![self.posts.a.clone(), self.posts.b.clone()],
				vec![self.tag_genealogist.clone()],
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations().collect::<Result<_, _>>()?;
			let expected_relations = hset! {
				Relation {
					post1: self.posts.a.clone(),
					post2: self.posts.b.clone(),
					score: self.posts.weighted_tag_score(&self.posts.a, &self.posts.b).into(),
				},
				Relation {
					post1: self.posts.b.clone(),
					post2: self.posts.a.clone(),
					score: self.posts.weighted_tag_score(&self.posts.b, &self.posts.a).into(),
				},
			};
			assert_eq!(expected_relations, relations);
			Ok(())
		}

		fn other_genealogist_two_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				vec![self.posts.a.clone(), self.posts.b.clone()],
				vec![self.link_genealogist.clone()],
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations().collect::<Result<_, _>>()?;
			let expected_relations = hset! {
				Relation {
					post1: self.posts.a.clone(),
					post2: self.posts.b.clone(),
					score: self.posts.weighted_link_score(&self.posts.a, &self.posts.b).into(),
				},
				Relation {
					post1: self.posts.b.clone(),
					post2: self.posts.a.clone(),
					score: self.posts.weighted_link_score(&self.posts.b, &self.posts.a).into(),
				},
			};
			assert_eq!(expected_relations, relations);

			Ok(())
		}

		fn one_genealogist_three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				vec![self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()],
				vec![self.tag_genealogist.clone()],
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
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
				let score = self.posts.weighted_tag_score(&post1, &post2).into();
				Relation::new(post1, post2, score).unwrap()
			})
			.collect::<HashSet<_>>();

			assert_eq!(expected_relations, relations);
			Ok(())
		}

		fn two_genealogists_three_posts(&self) -> Result<(), Exception> {
			let genealogy = Genealogy::new(
				vec![self.posts.a.clone(), self.posts.b.clone(), self.posts.c.clone()],
				vec![self.tag_genealogist.clone(), self.link_genealogist.clone()],
				self.weights.clone(),
			);

			let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
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
				let score = self.posts.link_and_tag_score(&post1, &post2).into();
				Relation::new(post1, post2, score).unwrap()
			})
			.collect::<HashSet<_>>();

			assert_eq!(expected_relations, relations);
			Ok(())
		}
	}

	#[derive(Clone)]
	struct Posts {
		a: Rc<Post>,
		b: Rc<Post>,
		c: Rc<Post>,
		tag_weight: Weight,
		link_weight: Weight,
	}

	impl Posts {
		pub fn new() -> Result<Posts, Exception> {
			Ok(Self {
				a: PostTestHelper::create_with_slug("a")?.into(),
				b: PostTestHelper::create_with_slug("b")?.into(),
				c: PostTestHelper::create_with_slug("c")?.into(),
				tag_weight: weight(1.0),
				link_weight: weight(0.75),
			})
		}

		fn tag_score(&self, post1: &Post, post2: &Post) -> i64 {
			if post1 == post2 {
				100
			} else if (post1 == self.a.as_ref()) && (post2 == self.b.as_ref()) {
				80
			} else if (post1 == self.a.as_ref()) && (post2 == self.c.as_ref()) {
				60
			} else if (post1 == self.b.as_ref()) && (post2 == self.a.as_ref()) {
				70
			} else if (post1 == self.b.as_ref()) && (post2 == self.c.as_ref()) {
				50
			} else if (post1 == self.c.as_ref()) && (post2 == self.a.as_ref()) {
				50
			} else if (post1 == self.c.as_ref()) && (post2 == self.b.as_ref()) {
				40
			} else {
				0
			}
		}

		fn weighted_tag_score(&self, post1: &Post, post2: &Post) -> WeightedScore {
			self.tag_score(post1, post2) * self.tag_weight
		}

		fn link_score(&self, post1: &Post, post2: &Post) -> i64 {
			if post1 == post2 {
				100
			} else if (post1 == self.a.as_ref()) && (post2 == self.b.as_ref()) {
				60
			} else if (post1 == self.a.as_ref()) && (post2 == self.c.as_ref()) {
				40
			} else if (post1 == self.b.as_ref()) && (post2 == self.a.as_ref()) {
				50
			} else if (post1 == self.b.as_ref()) && (post2 == self.c.as_ref()) {
				30
			} else if (post1 == self.c.as_ref()) && (post2 == self.a.as_ref()) {
				30
			} else if (post1 == self.c.as_ref()) && (post2 == self.b.as_ref()) {
				20
			} else {
				0
			}
		}

		fn weighted_link_score(&self, post1: &Post, post2: &Post) -> WeightedScore {
			self.link_score(post1, post2) * self.link_weight
		}

		fn link_and_tag_score(&self, post1: &Post, post2: &Post) -> i64 {
			vec![
				self.weighted_tag_score(post1, post2),
				self.weighted_link_score(post1, post2),
			]
			.into_iter()
			.collect::<Option<i64>>()
			.unwrap()
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
