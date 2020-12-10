use crate::genealogist::typed_relation::TypedRelation;
use crate::helpers::exception::Exception;
use crate::post::Post;
use std::sync::Arc;

pub mod genealogist_service;
pub mod relation_type;
pub mod typed_relation;

pub trait Genealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception>;
}

// NOTE: In Java this is automatically implemented
impl<Function> Genealogist for Function
where
	Function: Fn(Arc<Post>, Arc<Post>) -> Result<TypedRelation, Exception>,
{
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		self(post1, post2)
	}
}
