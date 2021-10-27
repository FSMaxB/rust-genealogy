use crate::genealogist::relation_type::RelationType;
use crate::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::IllegalArgumentException;
use genealogy_java_apis::string::JString;
use genealogy_java_apis::{record, throw};

/// ```java
/// public record TypedRelation(
///		Post post1,
///		Post post2,
///		RelationType type,
///		long score) {
/// ```
#[record(constructor = false)]
pub struct TypedRelation {
	post1: Post,
	post2: Post,
	r#type: RelationType,
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
}
