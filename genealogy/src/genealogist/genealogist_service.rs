use crate::genealogist::Genealogist;
use crate::post::Post;
use std::sync::Arc;

pub trait GenealogistService {
	fn procure(&self, posts: Box<dyn Iterator<Item = Arc<Post>>>) -> Arc<dyn Genealogist>;
}
