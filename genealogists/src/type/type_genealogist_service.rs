use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

pub struct TypeGenealogistService;

impl GenealogistService for TypeGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Result<Genealogist, Exception> {
		Ok(TypeGenealogist.into())
	}
}
