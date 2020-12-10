use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use std::sync::Arc;

pub struct RepoGenealogistService;

impl GenealogistService for RepoGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Arc<Post>>>) -> Arc<dyn Genealogist> {
		Arc::new(RepoGenealogist)
	}
}
