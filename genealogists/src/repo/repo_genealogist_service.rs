use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;

pub struct RepoGenealogistService;

impl GenealogistService for RepoGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Box<dyn Genealogist> {
		Box::new(RepoGenealogist)
	}
}
