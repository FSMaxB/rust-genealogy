use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::classes::GetClass;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::sync::Arc;

pub struct TypeGenealogist;

impl Genealogist for TypeGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		let score: u64 = match post2.get_class().get_simple_name() {
			"Article" => 50,
			"Video" => 90,
			"Talk" => 20,
			_ => 0,
		};

		TypedRelation::new(post1, post2, RelationType::from_value("type".to_string())?, score)
	}
}
