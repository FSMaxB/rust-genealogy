use crate::silly::silly_genealogist::SillyGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use genealogy_java_apis::collection::Collection;
use genealogy_java_apis::exception::Exception;

/// ```java
/// public class SillyGenealogistService implements GenealogistService {
/// ```
pub struct SillyGenealogistService;

/// ```java
/// public class SillyGenealogistService implements GenealogistService {
/// ```
impl GenealogistServiceTrait for SillyGenealogistService {
	/// ```java
	/// @Override
	///	public Genealogist procure(Collection<Post> posts) {
	///		return new SillyGenealogist();
	///	}
	/// ```
	#[allow(unused_variables)]
	fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception> {
		Ok(SillyGenealogist::new().into())
	}
}
