use crate::post::Post;
use std::sync::Arc;

pub mod recommender;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Recommendation {
	post: Arc<Post>,
	recommended_posts: Vec<Arc<Post>>,
}

impl Recommendation {
	pub fn new(
		post: Arc<Post>,
		sorted_recommendations: impl Iterator<Item = Arc<Post>>,
		per_post: usize,
	) -> Recommendation {
		let recommendations = sorted_recommendations.take(per_post).collect();
		Recommendation {
			post,
			recommended_posts: recommendations,
		}
	}

	pub fn post(&self) -> &Arc<Post> {
		&self.post
	}

	pub fn recommended_posts(&self) -> &Vec<Arc<Post>> {
		&self.recommended_posts
	}
}
