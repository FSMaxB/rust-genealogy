use crate::genealogist::Genealogist;
use crate::helpers::exception::Exception;
use crate::post::Post;

pub trait GenealogistService {
	fn procure(&self, posts: Box<dyn Iterator<Item = Post>>) -> Result<Genealogist, Exception>;
}
