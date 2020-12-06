use crate::genealogist::relation_type::RelationType;
use crate::java_replicas::exception::Exception;
use crate::java_replicas::exception::Exception::IllegalArgument;
use crate::post::Post;

#[derive(PartialEq, Eq, Hash)]
pub struct TypedRelation {
	// TODO: This will probably require some kind of reference type.
	pub post1: Post,
	pub post2: Post,
	// NOTE: `relation_type` instead of `type` because `type` is a keyword in rust.
	pub relation_type: RelationType,
	pub score: u64,
}

impl TypedRelation {
	#[allow(dead_code)]
	pub fn new(post1: Post, post2: Post, relation_type: RelationType, score: u64) -> Result<TypedRelation, Exception> {
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
