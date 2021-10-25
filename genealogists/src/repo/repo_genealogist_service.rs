use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::stream::Stream;
use genealogy::post::Post;

pub struct RepoGenealogistService;

impl GenealogistServiceTrait for RepoGenealogistService {
	fn procure(&self, _posts: Stream<Post>) -> Result<Genealogist, Exception> {
		Ok(RepoGenealogist.into())
	}
}
