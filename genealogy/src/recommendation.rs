use crate::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::list::List;
use genealogy_java_apis::record;
use genealogy_java_apis::stream::Stream;

pub mod recommender;

/// ```java
/// public record Recommendation(
///		Post post,
///		List<Post> recommendedPosts) {
///
/// 	public Recommendation {
///			requireNonNull(post);
///			requireNonNull(recommendedPosts);
///		}
/// ```
#[record]
pub struct Recommendation {
	post: Post,
	#[omit]
	recommended_posts: List<Post>,
}

impl Recommendation {
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
		List::copy_of(self.recommended_posts.clone())
	}
}
