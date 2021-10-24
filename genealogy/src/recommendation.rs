use crate::helpers::exception::Exception;
use crate::helpers::list::List;
use crate::helpers::stream::Stream;
use crate::post::Post;

pub mod recommender;

/// ```java
/// public record Recommendation(
///		Post post,
///		List<Post> recommendedPosts) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Recommendation {
	pub post: Post,
	pub recommended_posts: List<Post>,
}

impl Recommendation {
	/// ```java
	/// public Recommendation {
	///		requireNonNull(post);
	///		requireNonNull(recommendedPosts);
	///	}
	/// ```
	pub fn new(post: Post, recommended_posts: List<Post>) -> Self {
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
	pub(super) fn from(
		post: Post,
		sorted_recommendations: Stream<Post>,
		per_post: i32,
	) -> Result<Recommendation, Exception> {
		let recommendations = sorted_recommendations.limit(per_post)?.to_list()?;
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
	pub fn recommended_posts(&self) -> List<Post> {
		List::copy_of(&self.recommended_posts)
	}
}
