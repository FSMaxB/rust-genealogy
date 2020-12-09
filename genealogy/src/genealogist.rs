use crate::genealogist::typed_relation::TypedRelation;
use crate::helpers::exception::Exception;
use crate::post::Post;
use std::sync::Arc;

mod genealogist_service;
pub mod relation_type;
pub mod typed_relation;

pub trait Genealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception>;
}
