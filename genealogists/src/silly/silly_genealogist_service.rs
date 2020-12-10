use crate::silly::silly_genealogist::SillyGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use std::sync::Arc;

pub struct SillyGenealogistService;

impl GenealogistService for SillyGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Arc<Post>>>) -> Arc<dyn Genealogist> {
		Arc::new(SillyGenealogist)
	}
}
