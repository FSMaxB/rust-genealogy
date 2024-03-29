use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::GenealogistTrait;
use genealogy::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::r#static;
use std::fmt::{Display, Formatter};

/// ```java
/// public class TypeGenealogist implements Genealogist {
/// ```
#[derive(Debug)]
pub struct TypeGenealogist;

impl TypeGenealogist {
	// ```java
	// 	private static final RelationType TYPE = new RelationType("type");
	// ```
	r#static!(TYPE: RelationType = RelationType::new("type".into()).unwrap());

	/// ```java
	/// public class TypeGenealogist implements Genealogist {
	/// ```
	pub fn new() -> Self {
		Self
	}
}

/// ```java
/// public class TypeGenealogist implements Genealogist {
/// ```
impl GenealogistTrait for TypeGenealogist {
	/// ```java
	/// @Override
	///	public TypedRelation infer(Post post1, Post post2) {
	///		long score = switch (post2) {
	///			case Article __ -> 50;
	///			case Video __ -> 90;
	///			case Talk __ -> 20;
	///		};
	///
	///		return new TypedRelation(post1, post2, TYPE, score);
	///	}
	/// ```
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		use Post::*;
		let score = match post2 {
			Article(_) => 50,
			Video(_) => 90,
			Talk(_) => 20,
		};

		TypedRelation::new(post1, post2, Self::TYPE(), score)
	}
}

impl Display for TypeGenealogist {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		formatter.write_str("TypeGenealogist")
	}
}
