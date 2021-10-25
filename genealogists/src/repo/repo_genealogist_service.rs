use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::stream::Stream;
use genealogy::post::Post;

/// ```java
/// public class RepoGenealogistService implements GenealogistService {
/// ```
pub struct RepoGenealogistService;

/// ```java
/// public class RepoGenealogistService implements GenealogistService {
/// ```
impl GenealogistServiceTrait for RepoGenealogistService {
	/// ```java
	/// @Override
	///	public Genealogist procure(Collection<Post> posts) {
	///		return new RepoGenealogist();
	///	}
	/// ```
	#[allow(unused_variables)]
	fn procure(&self, posts: Stream<Post>) -> Result<Genealogist, Exception> {
		Ok(RepoGenealogist::new().into())
	}
}
