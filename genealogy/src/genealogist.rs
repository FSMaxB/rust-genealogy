use crate::genealogist::typed_relation::TypedRelation;
use crate::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::function::BiFunction;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod genealogist_service;
pub mod relation_type;
pub mod typed_relation;

/// ```java
/// public interface Genealogist {
/// ```
/// Type erased wrapper since in Java every interface is always automatically type erased.
#[derive(Clone)]
pub struct Genealogist {
	geneaologist: Rc<dyn GenealogistTrait>,
}

impl Genealogist {
	/// ```java
	/// 	TypedRelation infer(Post post1, Post post2);
	/// ```
	pub fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		self.geneaologist.infer(post1, post2)
	}
}

/// ```java
/// public interface Genealogist {
/// ```
pub trait GenealogistTrait: Display {
	/// ```java
	/// 	TypedRelation infer(Post post1, Post post2);
	/// ```
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception>;
}

// NOTE: In Java this is automatically implemented
impl GenealogistTrait for BiFunction<Post, Post, Result<TypedRelation, Exception>> {
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		self.apply(post1, post2)
	}
}

/// Helper to create instances of the type erased wrapper.
impl<GenealogistType> From<GenealogistType> for Genealogist
where
	GenealogistType: GenealogistTrait + 'static,
{
	fn from(genealogist: GenealogistType) -> Self {
		Self {
			geneaologist: Rc::new(genealogist),
		}
	}
}

impl Display for Genealogist {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		self.geneaologist.fmt(formatter)
	}
}
