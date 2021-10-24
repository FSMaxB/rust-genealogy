use crate::silly::silly_genealogist::SillyGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

pub struct SillyGenealogistService;

impl GenealogistService for SillyGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Result<Genealogist, Exception> {
		Ok(SillyGenealogist::new()?.into())
	}
}
