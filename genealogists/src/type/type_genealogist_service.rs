use crate::r#type::type_genealogist::TypeGenealogist;
use genealogy::genealogist::genealogist_service::GenealogistServiceTrait;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::collection::Collection;
use genealogy::helpers::exception::Exception;
use genealogy::post::Post;

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
