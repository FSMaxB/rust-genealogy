use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

pub struct TagGenealogist {
	r#type: RelationType,
}

impl TagGenealogist {
	pub fn new() -> Result<Self, Exception> {
		Ok(Self {
			r#type: RelationType::new("tag".to_string())?,
		})
	}
}

impl Genealogist for TagGenealogist {
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		let post2_tags = post2.tags();
		let number_of_shared_tags = post1.tags().iter().filter(|tag| post2_tags.contains(tag)).count() as u64;
		let number_of_post1_tags = post1.tags().len() as u64;
		let score = ((100.0 * 2.0 * number_of_shared_tags as f64)
			/ (((number_of_post1_tags as usize) + post2_tags.len()) as f64).round()) as i64;
		TypedRelation::new(post1, post2, self.r#type.clone(), score)
	}
}
