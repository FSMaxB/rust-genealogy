use crate::genealogist::relation_type::RelationType;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::post::Post;
use crate::throw;

/// ```java
/// public record TypedRelation(
///		Post post1,
///		Post post2,
///		RelationType type,
///		long score) {
/// ```
#[derive(PartialEq, Eq, Hash, Clone)] // FIXME: Clone or Inner with Rc?
pub struct TypedRelation {
	pub post1: Post,
	pub post2: Post,
	pub r#type: RelationType,
	score: i64,
}

impl TypedRelation {
	/// ```java
	/// 	public TypedRelation {
	///		requireNonNull(post1);
	///		requireNonNull(post2);
	///		requireNonNull(type);
	///		if (score < 0 || 100 < score)
	///			throw new IllegalArgumentException("Score should be in interval [0; 100]: " + score);
	///	}
	/// ```
	pub fn new(post1: Post, post2: Post, r#type: RelationType, score: i64) -> Result<Self, Exception> {
		#[allow(clippy::manual_range_contains)]
		if (score < 0) || (100 < score) {
			throw!(IllegalArgumentException(format!(
				"Score should be in interval [0; 100]: {}",
				score
			)));
		}

		Ok(TypedRelation {
			post1,
			post2,
			r#type,
			score,
		})
	}

	pub fn score(&self) -> i64 {
		self.score
	}
}
