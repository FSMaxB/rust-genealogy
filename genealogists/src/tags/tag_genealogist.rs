use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::GenealogistTrait;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use genealogy::r#static;

/// ```java
/// public class TagGenealogist implements Genealogist {
/// ```
pub struct TagGenealogist;

impl TagGenealogist {
	/// ```java
	/// public class TagGenealogist implements Genealogist {
	/// ```
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self
	}

	// ```java
	// private static final RelationType TYPE = new RelationType("tag");
	// ```
	r#static!(TYPE: RelationType = RelationType::new("tag".into()).unwrap());
}

/// ```java
/// public class TagGenealogist implements Genealogist {
/// ```
impl GenealogistTrait for TagGenealogist {
	/// ```java
	/// @Override
	///	public TypedRelation infer(Post post1, Post post2) {
	///		var post2Tags = post2.tags();
	///		long numberOfSharedTags = post1
	///				.tags().stream()
	///				.filter(post2Tags::contains)
	///				.count();
	///		long numberOfPost1Tags = post1.tags().size();
	///		long score = round((100.0 * 2 * numberOfSharedTags) / (numberOfPost1Tags + post2Tags.size()));
	///		return new TypedRelation(post1, post2, TYPE, score);
	///	}
	/// ```
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		let post2_tags = post2.tags();
		let number_of_shared_tags = post1
			.tags()
			.stream()
			.filter({
				let post2_tags = post2_tags.clone();
				move |tag| post2_tags.contains(tag)
			})
			.count()?;
		let number_of_post1_tags = post1.tags().size();
		let score = ((100.0 * 2.0 * (number_of_shared_tags as f64))
			/ ((number_of_post1_tags + post2_tags.size()) as f64))
			.round() as i64;
		TypedRelation::new(post1, post2, Self::TYPE(), score)
	}
}
