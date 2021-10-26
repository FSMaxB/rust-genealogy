use crate::genealogist::Genealogist;
use crate::post::Post;
use genealogy_java_apis::collection::Collection;
use genealogy_java_apis::exception::Exception;
use std::sync::Arc;

/// ```java
/// /**
///  * Used as a service to create {@link Genealogist}s - must have a public parameterless constructor.
///  */
/// public interface GenealogistService {
///		Genealogist procure(Collection<Post> posts);
/// }
/// ```
/// Type erased wrapper since in Java every interface is always automatically type erased.
#[derive(Clone)]
pub struct GenealogistService {
	genealogist_service: Arc<dyn GenealogistServiceTrait + Send + Sync>,
}

impl GenealogistService {
	/// ```java
	///	Genealogist procure(Collection<Post> posts);
	/// ```
	pub fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception> {
		self.genealogist_service.procure(posts)
	}
}

/// ```java
/// /**
///  * Used as a service to create {@link Genealogist}s - must have a public parameterless constructor.
///  */
/// public interface GenealogistService {
///		Genealogist procure(Collection<Post> posts);
/// }
/// ```
pub trait GenealogistServiceTrait {
	/// ```java
	///	Genealogist procure(Collection<Post> posts);
	/// ```
	fn procure(&self, posts: Collection<Post>) -> Result<Genealogist, Exception>;
}

/// Helper to create instance of the type erased wrapper.
impl<GenealogistServiceType> From<GenealogistServiceType> for GenealogistService
where
	GenealogistServiceType: GenealogistServiceTrait + Send + Sync + 'static,
{
	fn from(genealogist_service: GenealogistServiceType) -> Self {
		Self {
			genealogist_service: Arc::new(genealogist_service),
		}
	}
}
