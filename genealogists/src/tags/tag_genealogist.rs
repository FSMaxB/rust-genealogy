use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use lazy_static::lazy_static;
use std::sync::Arc;

pub struct TagGenealogist;

lazy_static! {
	static ref TYPE: RelationType = RelationType::from_value("tag".to_string()).unwrap();
}

impl Genealogist for TagGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		let post2_tags = post2.tags();
		let number_of_shared_tags = post1.tags().iter().filter(|tag| post2_tags.contains(tag)).count() as u64;
		let number_of_post1_tags = post1.tags().len() as u64;
		let score = ((100.0 * 2.0 * number_of_shared_tags as f64)
			/ (((number_of_post1_tags as usize) + post2_tags.len()) as f64))
			.round() as u64;
		TypedRelation::new(post1, post2, TYPE.clone(), score)
	}
}
