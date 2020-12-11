use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::ops::Deref;
use std::sync::Arc;

pub struct TypeGenealogist;

impl Genealogist for TypeGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		use Post::*;
		let score: u64 = match post2.deref() {
			Article(_) => 50,
			Video(_) => 90,
			Talk(_) => 20,
		};

		TypedRelation::new(post1, post2, RelationType::from_value("type".to_string())?, score)
	}
}
