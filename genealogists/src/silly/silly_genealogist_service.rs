use crate::silly::silly_genealogist::SillyGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::rc::Rc;

pub struct SillyGenealogistService;

impl GenealogistService for SillyGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Rc<Post>>>) -> Result<Rc<dyn Genealogist>, Exception> {
		Ok(Rc::new(SillyGenealogist::new()?))
	}
}
