use crate::tags::tag_genealogist::TagGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::collection::Collection;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

/// ```java
/// public class TagGenealogistService implements GenealogistService {
/// ```
pub struct TagGenealogistService;

/// ```java
/// public class TagGenealogistService implements GenealogistService {
/// ```
impl GenealogistServiceTrait for TagGenealogistService {
	/// ```java
	/// @Override
	///	public Genealogist procure(Collection<Post> posts) {
	///		return new TagGenealogist();
	///	}
	/// ```
	#[allow(unused_variables)]
	fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception> {
		Ok(TagGenealogist::new().into())
	}
}
