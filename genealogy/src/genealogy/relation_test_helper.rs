use crate::genealogy::relation::Relation;
use crate::post::Post;
use genealogy_java_apis::exception::Exception;

/// ```java
/// public class RelationTestHelper {
/// ```
pub struct RelationTestHelper;

impl RelationTestHelper {
	/// ```java
	/// public static Relation create(Post post1, Post post2, long score) {
	/// 	return new Relation(post1, post2, score);
	/// }
	/// ```
	pub fn create(post1: Post, post2: Post, score: i64) -> Result<Relation, Exception> {
		Relation::new(post1, post2, score)
	}
}
