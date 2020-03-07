use crate::article::Article;
use crate::genealogist::typed_relation::TypedRelation;

pub trait Genealogist {
	fn infer(&self, article1: Article, article2: Article) -> TypedRelation;
}
