use crate::tags::tag_genealogist::TagGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::stream::Stream;
use genealogy::post::Post;

pub struct TagGenealogistService;

impl GenealogistServiceTrait for TagGenealogistService {
	fn procure(&self, _posts: Stream<Post>) -> Result<Genealogist, Exception> {
		Ok(TagGenealogist::new()?.into())
	}
}
