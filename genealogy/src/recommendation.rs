use crate::helpers::exception::Exception;
use crate::helpers::stream::Stream;
use crate::post::Post;
use std::rc::Rc;

pub mod recommender;

/// ```java
/// public record Recommendation(
///		Post post,
///		List<Post> recommendedPosts) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Recommendation {
	pub post: Rc<Post>,
	pub recommended_posts: Vec<Rc<Post>>,
}

impl Recommendation {
	/// ```java
	/// public Recommendation {
	///		requireNonNull(post);
	///		requireNonNull(recommendedPosts);
	///	}
	/// ```
	pub fn new(post: Rc<Post>, recommended_posts: Vec<Rc<Post>>) -> Self {
		Self {
			post,
			recommended_posts,
		}
	}

	/// ```java
	/// static Recommendation from(Post post, Stream<Post> sortedRecommendations, int perPost) {
	///		var recommendations = sortedRecommendations.limit(perPost).toList();
	///		return new Recommendation(requireNonNull(post), recommendations);
	///	}
	/// ```
	pub fn from(
		post: Rc<Post>,
		sorted_recommendations: Stream<Rc<Post>>,
		per_post: usize,
	) -> Result<Recommendation, Exception> {
		let recommendations = sorted_recommendations.limit(per_post).to_list()?;
		Ok(Recommendation {
			post,
			recommended_posts: recommendations,
		})
	}

	/// ```java
	/// public List<Post> recommendedPosts() {
	///		return List.copyOf(recommendedPosts);
	///	}
	/// ```
	pub fn recommended_posts(&self) -> Vec<Rc<Post>> {
		self.recommended_posts.clone()
	}
}
