use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::post::Post;
use genealogy_java_apis::collection::Collection;
use genealogy_java_apis::exception::Exception;

/// ```java
/// public class TypeGenealogistService implements GenealogistService {
/// ```
pub struct TypeGenealogistService;

/// ```java
/// public class TypeGenealogistService implements GenealogistService {
/// ```
impl GenealogistServiceTrait for TypeGenealogistService {
	/// ```java
	/// @Override
	///	public Genealogist procure(Collection<Post> posts) {
	///		return new TypeGenealogist();
	///	}
	/// ```
	#[allow(unused_variables)]
	fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception> {
		Ok(TypeGenealogist::new().into())
	}
}
