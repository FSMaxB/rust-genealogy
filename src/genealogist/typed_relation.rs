use crate::article::Article;
use crate::exception::Exception;
use crate::genealogist::relation_type::RelationType;

pub struct TypedRelation {
	pub article1: Article,
	pub article2: Article,
	pub r#type: RelationType,
	pub score: u64,
}

impl TypedRelation {
	pub fn new(
		article1: Article,
		article2: Article,
		r#type: RelationType,
		score: u64,
	) -> Result<TypedRelation, Exception> {
		if !(1u64..100).contains(&score) {
			Err(Exception::IllegalArgument(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)))
		} else {
			Ok(TypedRelation {
				article1,
				article2,
				r#type,
				score,
			})
		}
	}
}
