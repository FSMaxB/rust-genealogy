use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;
use std::rc::Rc;

pub struct RepoGenealogistService;

impl GenealogistService for RepoGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Rc<Post>>>) -> Result<Rc<dyn Genealogist>, Exception> {
		Ok(Rc::new(RepoGenealogist))
	}
}
