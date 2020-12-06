use crate::genealogist::Genealogist;
use crate::post::Post;

pub trait GenealogistService {
	fn procure(&self, posts: Box<dyn Iterator<Item = Post>>) -> Box<dyn Genealogist>;
}
