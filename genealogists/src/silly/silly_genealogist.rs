use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::collections::BTreeSet;
use std::rc::Rc;

pub struct SillyGenealogist {
	r#type: RelationType,
}

impl SillyGenealogist {
	pub fn new() -> Result<Self, Exception> {
		Ok(Self {
			r#type: RelationType::new("silly".to_string())?,
		})
	}
}

impl Genealogist for SillyGenealogist {
	fn infer(&self, post1: Rc<Post>, post2: Rc<Post>) -> Result<TypedRelation, Exception> {
		let post1_letters = title_letters(&post1);
		let post2_letters = title_letters(&post2);
		let intersection = post1_letters.intersection(&post2_letters);
		let score = ((100.0 * intersection.count() as f64) / (post1_letters.len() as f64)).round() as i64;

		TypedRelation::new(post1, post2, self.r#type.clone(), score)
	}
}

fn title_letters(post: &Post) -> BTreeSet<u16> {
	post.title().text.to_lowercase().encode_utf16().collect()
}
