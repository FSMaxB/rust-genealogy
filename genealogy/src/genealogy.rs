use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogist::Genealogist;
use crate::genealogy::relation::Relation;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::iterator::result_iterator::ResultIteratorExtension;
use crate::post::Post;
use resiter::Map;
use std::collections::HashMap;
use std::sync::Arc;

pub mod relation;
pub mod score;
pub mod weight;
pub mod weights;

pub struct Genealogy {
	posts: Vec<Arc<Post>>,
	genealogists: Vec<Arc<dyn Genealogist>>,
	weights: Arc<Weights>,
}

impl Genealogy {
	pub fn new(posts: Vec<Arc<Post>>, genealogists: Vec<Arc<dyn Genealogist>>, weights: Arc<Weights>) -> Self {
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
	posts: Vec<Arc<Post>>,
	genealogists: Vec<Arc<dyn Genealogist>>,
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
	use crate::genealogy::score::{score, Score, WeightedScore};
	use crate::genealogy::weight::{weight, Weight};
	use crate::post::test::PostTestHelper;
	use lazy_static::lazy_static;
	use literally::{hmap, hset};
	use std::collections::HashSet;

	lazy_static! {
		static ref TAG_WEIGHT: Weight = weight(1.0);
		static ref LINK_WEIGHT: Weight = weight(0.75);
		static ref POST_A: Arc<Post> = PostTestHelper::create_with_slug("a").unwrap().into();
		static ref POST_B: Arc<Post> = PostTestHelper::create_with_slug("b").unwrap().into();
		static ref POST_C: Arc<Post> = PostTestHelper::create_with_slug("c").unwrap().into();
		static ref TAG_RELATION: RelationType = RelationType::new("tag".to_string()).unwrap();
		static ref LINK_RELATION: RelationType = RelationType::new("link".to_string()).unwrap();
		static ref TAG_GENEALOGIST: Arc<dyn Genealogist + Send + Sync> =
			Arc::new(|post1: Arc<Post>, post2: Arc<Post>| {
				let score = tag_score(&post1, &post2);
				Ok(TypedRelation {
					post1,
					post2,
					relation_type: TAG_RELATION.clone(),
					score,
				})
			});
		static ref LINK_GENEALOGIST: Arc<dyn Genealogist + Send + Sync> =
			Arc::new(|post1: Arc<Post>, post2: Arc<Post>| {
				let score = link_score(&post1, &post2);
				Ok(TypedRelation {
					post1,
					post2,
					relation_type: LINK_RELATION.clone(),
					score,
				})
			});
		static ref WEIGHTS: Arc<Weights> = Arc::new(Weights::new(
			hmap! {
				TAG_RELATION.clone() => *TAG_WEIGHT,
				LINK_RELATION.clone() => *LINK_WEIGHT,
			},
			weight(0.5),
		));
	}

	fn tag_score(post1: &Post, post2: &Post) -> Score {
		// TODO: Use Match?
		if post1 == post2 {
			score(100)
		} else if (post1 == POST_A.as_ref()) && (post2 == POST_B.as_ref()) {
			score(80)
		} else if (post1 == POST_A.as_ref()) && (post2 == POST_C.as_ref()) {
			score(60)
		} else if (post1 == POST_B.as_ref()) && (post2 == POST_A.as_ref()) {
			score(70)
		} else if (post1 == POST_B.as_ref()) && (post2 == POST_C.as_ref()) {
			score(50)
		} else if (post1 == POST_C.as_ref()) && (post2 == POST_A.as_ref()) {
			score(50)
		} else if (post1 == POST_C.as_ref()) && (post2 == POST_B.as_ref()) {
			score(40)
		} else {
			score(0)
		}
	}

	fn weighted_tag_score(post1: &Post, post2: &Post) -> WeightedScore {
		tag_score(post1, post2) * *TAG_WEIGHT
	}

	fn link_score(post1: &Post, post2: &Post) -> Score {
		if post1 == post2 {
			score(100)
		} else if (post1 == POST_A.as_ref()) && (post2 == POST_B.as_ref()) {
			score(60)
		} else if (post1 == POST_A.as_ref()) && (post2 == POST_C.as_ref()) {
			score(40)
		} else if (post1 == POST_B.as_ref()) && (post2 == POST_A.as_ref()) {
			score(50)
		} else if (post1 == POST_B.as_ref()) && (post2 == POST_C.as_ref()) {
			score(30)
		} else if (post1 == POST_C.as_ref()) && (post2 == POST_A.as_ref()) {
			score(30)
		} else if (post1 == POST_C.as_ref()) && (post2 == POST_B.as_ref()) {
			score(20)
		} else {
			score(0)
		}
	}

	fn weighted_link_score(post1: &Post, post2: &Post) -> WeightedScore {
		link_score(post1, post2) * *LINK_WEIGHT
	}

	#[test]
	fn one_genealogist_two_posts() {
		let genealogy = Genealogy::new(
			vec![POST_A.clone(), POST_B.clone()],
			vec![TAG_GENEALOGIST.clone()],
			WEIGHTS.clone(),
		);

		let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
		let expected_relations = hset! {
			Relation {
				post1: POST_A.clone(),
				post2: POST_B.clone(),
				score: weighted_tag_score(&POST_A, &POST_B).into(),
			},
			Relation {
				post1: POST_B.clone(),
				post2: POST_A.clone(),
				score: weighted_tag_score(&POST_B, &POST_A).into(),
			},
		};
		assert_eq!(expected_relations, relations);
	}

	#[test]
	fn other_genealogist_two_posts() {
		let genealogy = Genealogy::new(
			vec![POST_A.clone(), POST_B.clone()],
			vec![LINK_GENEALOGIST.clone()],
			WEIGHTS.clone(),
		);

		let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
		let expected_relations = hset! {
			Relation {
				post1: POST_A.clone(),
				post2: POST_B.clone(),
				score: weighted_link_score(&POST_A, &POST_B).into(),
			},
			Relation {
				post1: POST_B.clone(),
				post2: POST_A.clone(),
				score: weighted_link_score(&POST_B, &POST_A).into(),
			},
		};
		assert_eq!(expected_relations, relations);
	}

	#[test]
	fn one_genealogist_three_posts() {
		let genealogy = Genealogy::new(
			vec![POST_A.clone(), POST_B.clone(), POST_C.clone()],
			vec![TAG_GENEALOGIST.clone()],
			WEIGHTS.clone(),
		);

		let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
		let expected_relations = vec![
			(POST_A.clone(), POST_B.clone()),
			(POST_A.clone(), POST_C.clone()),
			(POST_B.clone(), POST_A.clone()),
			(POST_B.clone(), POST_C.clone()),
			(POST_C.clone(), POST_A.clone()),
			(POST_C.clone(), POST_B.clone()),
		]
		.into_iter()
		.map(|(post1, post2)| {
			let score = weighted_tag_score(&post1, &post2).into();
			Relation { post1, post2, score }
		})
		.collect::<HashSet<_>>();

		assert_eq!(expected_relations, relations);
	}

	#[test]
	fn two_genealogists_three_posts() {
		let genealogy = Genealogy::new(
			vec![POST_A.clone(), POST_B.clone(), POST_C.clone()],
			vec![TAG_GENEALOGIST.clone(), LINK_GENEALOGIST.clone()],
			WEIGHTS.clone(),
		);

		let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
		let expected_relations = vec![
			(POST_A.clone(), POST_B.clone()),
			(POST_A.clone(), POST_C.clone()),
			(POST_B.clone(), POST_A.clone()),
			(POST_B.clone(), POST_C.clone()),
			(POST_C.clone(), POST_A.clone()),
			(POST_C.clone(), POST_B.clone()),
		]
		.into_iter()
		.map(|(post1, post2)| {
			let score = link_and_tag_score(&post1, &post2).into();
			Relation { post1, post2, score }
		})
		.collect::<HashSet<_>>();

		assert_eq!(expected_relations, relations);
	}

	fn link_and_tag_score(post1: &Post, post2: &Post) -> Score {
		vec![weighted_tag_score(post1, post2), weighted_link_score(post1, post2)]
			.into_iter()
			.collect::<Option<Score>>()
			.unwrap()
	}
}
