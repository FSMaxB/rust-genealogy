use crate::genealogist::relation_type::RelationType;
use crate::genealogy::score::Score;
use crate::post::Post;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash)]
pub struct TypedRelation {
	pub post1: Arc<Post>,
	pub post2: Arc<Post>,
	// NOTE: `relation_type` instead of `type` because `type` is a keyword in rust.
	pub relation_type: RelationType,
	pub score: Score,
}
