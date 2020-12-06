use crate::genealogist::typed_relation::TypedRelation;
use crate::post::Post;

mod genealogist_service;
pub mod relation_type;
mod typed_relation;

pub trait Genealogist {
	fn infer(&self, post1: Post, post2: Post) -> TypedRelation;
}
