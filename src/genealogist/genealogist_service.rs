use crate::article::Article;
use crate::genealogist::genealogist::Genealogist;

pub trait GenealogistService {
	fn procure(&self, articles: &mut dyn Iterator<Item = Article>) -> Box<dyn Genealogist>;
}
