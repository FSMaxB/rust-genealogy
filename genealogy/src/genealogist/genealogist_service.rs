use crate::genealogist::Genealogist;
use crate::helpers::exception::Exception;
use crate::helpers::stream::Stream;
use crate::post::Post;

/// ```java
/// /**
///  * Used as a service to create {@link Genealogist}s - must have a public parameterless constructor.
///  */
/// public interface GenealogistService {
///		Genealogist procure(Collection<Post> posts);
/// }
/// ```
/// Type erased wrapper since in Java every interface is always automatically type erased.
pub struct GenealogistService {
	genealogist_service: Box<dyn GenealogistServiceTrait>,
}

impl GenealogistService {
	/// ```java
	///	Genealogist procure(Collection<Post> posts);
	/// ```
	pub fn procure(&self, posts: Stream<Post>) -> Result<Genealogist, Exception> {
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
	fn procure(&self, posts: Stream<Post>) -> Result<Genealogist, Exception>;
}

/// Helper to create instance of the type erased wrapper.
impl<GenealogistServiceType> From<GenealogistServiceType> for GenealogistService
where
	GenealogistServiceType: GenealogistServiceTrait + 'static,
{
	fn from(genealogist_service: GenealogistServiceType) -> Self {
		Self {
			genealogist_service: Box::new(genealogist_service),
		}
	}
}
