use crate::genealogist::typed_relation::TypedRelation;
use crate::post::Post;

pub trait Genealogist {
	fn infer(&self, post1: Post, post2: Post) -> TypedRelation;
}
