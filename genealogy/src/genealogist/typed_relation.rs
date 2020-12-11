use crate::genealogist::relation_type::RelationType;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;
use crate::post::Post;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash)]
pub struct TypedRelation {
	pub post1: Arc<Post>,
	pub post2: Arc<Post>,
	// NOTE: `relation_type` instead of `type` because `type` is a keyword in rust.
	pub relation_type: RelationType,
	pub score: u64,
}

impl TypedRelation {
	pub fn new(
		post1: Arc<Post>,
		post2: Arc<Post>,
		relation_type: RelationType,
		score: u64,
	) -> Result<TypedRelation, Exception> {
		// WTF: Why use a `long` if you are actually only allowing values up to 100?
		// RUSTIFICATION: Create a separate type that upholds this invariant.
		if score > 100 {
			Err(IllegalArgument(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)))
		} else {
			Ok(TypedRelation {
				post1,
				post2,
				relation_type,
				score,
			})
		}
	}
}
