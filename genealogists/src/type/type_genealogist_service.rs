use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::rc::Rc;

pub struct TypeGenealogistService;

impl GenealogistService for TypeGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Rc<Post>>>) -> Result<Rc<dyn Genealogist>, Exception> {
		Ok(Rc::new(TypeGenealogist))
	}
}
