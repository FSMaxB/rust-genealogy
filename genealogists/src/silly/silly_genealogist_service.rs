use crate::silly::silly_genealogist::SillyGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::stream::Stream;
use genealogy::post::Post;

pub struct SillyGenealogistService;

impl GenealogistServiceTrait for SillyGenealogistService {
	fn procure(&self, _posts: Stream<Post>) -> Result<Genealogist, Exception> {
		Ok(SillyGenealogist::new()?.into())
	}
}
