use crate::genealogist::typed_relation::TypedRelation;
use crate::helpers::exception::Exception;
use crate::post::Post;

pub mod genealogist_service;
pub mod relation_type;
pub mod typed_relation;

pub trait Genealogist {
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception>;
}

// NOTE: In Java this is automatically implemented
impl<Function> Genealogist for Function
where
	Function: Fn(Post, Post) -> Result<TypedRelation, Exception>,
{
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		self(post1, post2)
	}
}
