use crate::genealogist::relation_type::RelationType;
use crate::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::IllegalArgumentException;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::throw;

/// ```java
/// public record TypedRelation(
///		Post post1,
///		Post post2,
///		RelationType type,
///		long score) {
/// ```
// FIXME: Make constructor optional in the #[record] macro so it can be used here
#[derive(PartialEq, Eq, Hash, Clone)]
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
			throw!(IllegalArgumentException(
				JString::from("Score should be in interval [0; 100]: ") + score
			));
		}

		Ok(TypedRelation {
			post1,
			post2,
			r#type,
			score,
		})
	}

	pub fn post1(&self) -> Post {
		self.post1.clone()
	}

	pub fn post2(&self) -> Post {
		self.post2.clone()
	}

	pub fn score(&self) -> i64 {
		self.score
	}
}
