use crate::genealogist::typed_relation::TypedRelation;
use crate::helpers::exception::Exception;
use crate::post::Post;
use std::rc::Rc;

pub mod genealogist_service;
pub mod relation_type;
pub mod typed_relation;

pub trait Genealogist {
	fn infer(&self, post1: Rc<Post>, post2: Rc<Post>) -> Result<TypedRelation, Exception>;
}

// NOTE: In Java this is automatically implemented
impl<Function> Genealogist for Function
where
	Function: Fn(Rc<Post>, Rc<Post>) -> Result<TypedRelation, Exception>,
{
	fn infer(&self, post1: Rc<Post>, post2: Rc<Post>) -> Result<TypedRelation, Exception> {
		self(post1, post2)
	}
}
