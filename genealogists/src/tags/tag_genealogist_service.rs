use crate::tags::tag_genealogist::TagGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use std::sync::Arc;

pub struct TagGenealogistService;

impl GenealogistService for TagGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Arc<Post>>>) -> Arc<dyn Genealogist> {
		Arc::new(TagGenealogist)
	}
}
