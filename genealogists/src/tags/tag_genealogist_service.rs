use crate::tags::tag_genealogist::TagGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::rc::Rc;

pub struct TagGenealogistService;

impl GenealogistService for TagGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Result<Rc<dyn Genealogist>, Exception> {
		Ok(Rc::new(TagGenealogist::new()?))
	}
}
