use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;

pub struct TypeGenealogistService;

impl GenealogistService for TypeGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Box<dyn Genealogist> {
		Box::new(TypeGenealogist)
	}
}
