use crate::tags::tag_genealogist::TagGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistService;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

pub struct TagGenealogistService;

impl GenealogistService for TagGenealogistService {
	fn procure(&self, _posts: Box<dyn Iterator<Item = Post>>) -> Result<Genealogist, Exception> {
		Ok(TagGenealogist::new()?.into())
	}
}
