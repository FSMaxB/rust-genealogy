use crate::post::Post;
use std::num::NonZeroUsize;
use std::rc::Rc;

pub mod recommender;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Recommendation {
	post: Rc<Post>,
	recommended_posts: Vec<Rc<Post>>,
}

impl Recommendation {
	pub fn new(
		post: Rc<Post>,
		sorted_recommendations: impl Iterator<Item = Rc<Post>>,
		per_post: NonZeroUsize,
	) -> Recommendation {
		let recommendations = sorted_recommendations.take(per_post.get()).collect();
		Recommendation {
			post,
			recommended_posts: recommendations,
		}
	}

	pub fn post(&self) -> &Rc<Post> {
		&self.post
	}

	pub fn recommended_posts(&self) -> &Vec<Rc<Post>> {
		&self.recommended_posts
	}
}
