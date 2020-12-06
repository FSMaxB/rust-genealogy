use crate::post::Post;
use std::sync::Arc;

#[derive(PartialEq, Eq, Hash)]
pub struct Recommendation {
	pub post: Arc<Post>,
	pub recommended_posts: Vec<Arc<Post>>,
}

impl Recommendation {
	#[allow(dead_code)]
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
}
