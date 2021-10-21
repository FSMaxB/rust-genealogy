use crate::genealogist::Genealogist;
use crate::helpers::exception::Exception;
use crate::post::Post;
use std::rc::Rc;

pub trait GenealogistService {
	fn procure(&self, posts: Box<dyn Iterator<Item = Post>>) -> Result<Rc<dyn Genealogist>, Exception>;
}
