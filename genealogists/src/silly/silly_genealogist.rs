use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::genealogy::score::Score;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use lazy_static::lazy_static;
use std::collections::BTreeSet;
use std::sync::Arc;

pub struct SillyGenealogist;

lazy_static! {
	static ref TYPE: RelationType = RelationType::from_value("silly".to_string()).unwrap();
}

impl Genealogist for SillyGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		let post1_letters = title_letters(&post1);
		let post2_letters = title_letters(&post2);
		let intersection = post1_letters.intersection(&post2_letters);
		let score =
			Score::try_from(((100.0 * intersection.count() as f64) / (post1_letters.len() as f64)).round()).unwrap();

		Ok(TypedRelation {
			post1,
			post2,
			relation_type: TYPE.clone(),
			score,
		})
	}
}

fn title_letters(post: &Post) -> BTreeSet<u16> {
	post.title().text.to_lowercase().encode_utf16().collect()
}
