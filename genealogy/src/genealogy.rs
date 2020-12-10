use crate::genealogist::typed_relation::TypedRelation;
use crate::genealogist::Genealogist;
use crate::genealogy::relation::Relation;
use crate::genealogy::weights::Weights;
use crate::helpers::exception::Exception;
use crate::helpers::iterator::result_iterator::ResultIteratorExtension;
use crate::post::Post;
use resiter::Map;
use std::collections::BTreeMap;
use std::sync::Arc;

pub mod relation;
pub mod weights;

pub struct Genealogy {
	posts: Vec<Arc<Post>>,
	genealogists: Vec<Arc<dyn Genealogist>>,
	weights: Arc<Weights>,
}

impl Genealogy {
	#[allow(dead_code)]
	pub fn new(posts: Vec<Arc<Post>>, genealogists: Vec<Arc<dyn Genealogist>>, weights: Arc<Weights>) -> Self {
		Self {
			posts,
			genealogists,
			weights,
		}
	}

	#[allow(dead_code)]
	pub fn infer_relations(&self) -> impl Iterator<Item = Result<Relation, Exception>> {
		self.aggregate_typed_relations(infer_typed_relations(self.posts.clone(), self.genealogists.clone()))
	}

	fn aggregate_typed_relations(
		&self,
		mut typed_relations: impl Iterator<Item = Result<TypedRelation, Exception>>,
	) -> impl Iterator<Item = Result<Relation, Exception>> {
		let sorted_typed_relations = typed_relations.try_fold(BTreeMap::new(), |mut map, result| {
			let relation = result?;
			map.entry(relation.post1.clone())
				.or_insert_with(BTreeMap::new)
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
	use crate::post::test::post_with_slug;
	use lazy_static::lazy_static;
	use literally::{bmap, bset};

	// NOTE: Using f64 directly saves a lot of casting
	const TAG_SCORE_A_B: f64 = 80.0;
	const TAG_SCORE_A_C: f64 = 60.0;
	const TAG_SCORE_B_A: f64 = 70.0;
	const TAG_SCORE_B_C: f64 = 50.0;
	const TAG_SCORE_C_A: f64 = 50.0;
	const TAG_SCORE_C_B: f64 = 40.0;

	const LINK_SCORE_A_B: f64 = 60.0;
	const LINK_SCORE_A_C: f64 = 40.0;
	const LINK_SCORE_B_A: f64 = 50.0;
	const LINK_SCORE_B_C: f64 = 30.0;
	const LINK_SCORE_C_A: f64 = 30.0;
	const LINK_SCORE_C_B: f64 = 20.0;

	const TAG_WEIGHT: f64 = 1.0;
	const LINK_WEIGHT: f64 = 0.75;

	lazy_static! {
		static ref POST_A: Arc<Post> = post_with_slug("a").unwrap().into();
		static ref POST_B: Arc<Post> = post_with_slug("b").unwrap().into();
		static ref POST_C: Arc<Post> = post_with_slug("c").unwrap().into();
		static ref TAG_RELATION: RelationType = RelationType::from_value("tag".to_string()).unwrap();
		static ref LINK_RELATION: RelationType = RelationType::from_value("link".to_string()).unwrap();
		static ref TAG_GENEALOGIST: Arc<dyn Genealogist + Send + Sync> =
			Arc::new(|post1: Arc<Post>, post2: Arc<Post>| {
				let score = tag_score(&post1, &post2);
				TypedRelation::new(post1, post2, TAG_RELATION.clone(), score as u64)
			});
		static ref LINK_GENEALOGIST: Arc<dyn Genealogist + Send + Sync> =
			Arc::new(|post1: Arc<Post>, post2: Arc<Post>| {
				let score = link_score(&post1, &post2);
				TypedRelation::new(post1, post2, LINK_RELATION.clone(), score as u64)
			});
		static ref WEIGHTS: Arc<Weights> = Arc::new(Weights::new(
			bmap! {
				TAG_RELATION.clone() => TAG_WEIGHT,
				LINK_RELATION.clone() => LINK_WEIGHT,
			},
			0.5
		));
	}

	fn tag_score(post1: &Post, post2: &Post) -> f64 {
		if post1 == post2 {
			return 100.0;
		}

		if (post1 == POST_A.as_ref()) && (post2 == POST_B.as_ref()) {
			return TAG_SCORE_A_B;
		}

		if (post1 == POST_A.as_ref()) && (post2 == POST_C.as_ref()) {
			return TAG_SCORE_A_C;
		}

		if (post1 == POST_B.as_ref()) && (post2 == POST_A.as_ref()) {
			return TAG_SCORE_B_A;
		}

		if (post1 == POST_B.as_ref()) && (post2 == POST_C.as_ref()) {
			return TAG_SCORE_B_C;
		}

		if (post1 == POST_C.as_ref()) && (post2 == POST_A.as_ref()) {
			return TAG_SCORE_C_A;
		}

		if (post1 == POST_C.as_ref()) && (post2 == POST_B.as_ref()) {
			return TAG_SCORE_C_B;
		}

		0.0
	}

	fn link_score(post1: &Post, post2: &Post) -> f64 {
		if post1 == post2 {
			return 100.0;
		}

		if (post1 == POST_A.as_ref()) && (post2 == POST_B.as_ref()) {
			return LINK_SCORE_A_B;
		}

		if (post1 == POST_A.as_ref()) && (post2 == POST_C.as_ref()) {
			return LINK_SCORE_A_C;
		}

		if (post1 == POST_B.as_ref()) && (post2 == POST_A.as_ref()) {
			return LINK_SCORE_B_A;
		}

		if (post1 == POST_B.as_ref()) && (post2 == POST_C.as_ref()) {
			return LINK_SCORE_B_C;
		}

		if (post1 == POST_C.as_ref()) && (post2 == POST_A.as_ref()) {
			return LINK_SCORE_C_A;
		}

		if (post1 == POST_C.as_ref()) && (post2 == POST_B.as_ref()) {
			return LINK_SCORE_C_B;
		}

		0.0
	}

	#[test]
	fn one_genealogist_two_posts() {
		let genealogy = Genealogy::new(
			vec![POST_A.clone(), POST_B.clone()],
			vec![TAG_GENEALOGIST.clone()],
			WEIGHTS.clone(),
		);

		let relations = genealogy.infer_relations().collect::<Result<_, _>>().unwrap();
		let expected_relations = bset! {
			Relation::new(
				POST_A.clone(),
				POST_B.clone(),
				(TAG_SCORE_A_B * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_A.clone(),
				(TAG_SCORE_B_A * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
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
		let expected_relations = bset! {
			Relation::new(
				POST_A.clone(),
				POST_B.clone(),
				((LINK_SCORE_A_B) * LINK_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_A.clone(),
				((LINK_SCORE_B_A) * LINK_WEIGHT).round() as u64,
			)
			.unwrap(),
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
		// RUSTIFICATION: Create these values from a simpler list of elements like e.g. ("a", "b")
		let expected_relations = bset! {
			Relation::new(
				POST_A.clone(),
				POST_B.clone(),
				((TAG_SCORE_A_B) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_A.clone(),
				POST_C.clone(),
				((TAG_SCORE_A_C) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_A.clone(),
				((TAG_SCORE_B_A) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_C.clone(),
				((TAG_SCORE_B_C) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_C.clone(),
				POST_A.clone(),
				((TAG_SCORE_C_A) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_C.clone(),
				POST_B.clone(),
				((TAG_SCORE_C_B) * TAG_WEIGHT).round() as u64,
			)
			.unwrap(),
		};

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
		let expected_relations = bset! {
			Relation::new(
				POST_A.clone(),
				POST_B.clone(),
				(((TAG_SCORE_A_B * TAG_WEIGHT) + (LINK_SCORE_A_B * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_A.clone(),
				POST_C.clone(),
				(((TAG_SCORE_A_C * TAG_WEIGHT) + (LINK_SCORE_A_C * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_A.clone(),
				(((TAG_SCORE_B_A * TAG_WEIGHT) + (LINK_SCORE_B_A * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_B.clone(),
				POST_C.clone(),
				(((TAG_SCORE_B_C * TAG_WEIGHT) + (LINK_SCORE_B_C * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_C.clone(),
				POST_A.clone(),
				(((TAG_SCORE_C_A * TAG_WEIGHT) + (LINK_SCORE_C_A * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
			Relation::new(
				POST_C.clone(),
				POST_B.clone(),
				(((TAG_SCORE_C_B * TAG_WEIGHT) + (LINK_SCORE_C_B * LINK_WEIGHT)) / 2.0).round() as u64,
			)
			.unwrap(),
		};

		assert_eq!(expected_relations, relations);
	}
}
