use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::stream::Stream;
use genealogy::post::Post;

pub struct TypeGenealogistService;

impl GenealogistServiceTrait for TypeGenealogistService {
	fn procure(&self, _posts: Stream<Post>) -> Result<Genealogist, Exception> {
		Ok(TypeGenealogist.into())
	}
}
