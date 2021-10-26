use crate::repo::repo_genealogist::RepoGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use genealogy_java_apis::collection::Collection;
use genealogy_java_apis::exception::Exception;

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
	fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception> {
		Ok(RepoGenealogist::new().into())
	}
}
