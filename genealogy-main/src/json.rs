use genealogy::post::Post;
use genealogy::recommendation::Recommendation;
use serde::Serialize;
use std::iter::FromIterator;

#[derive(Serialize)]
pub struct SerializedRecommendations {
	#[serde(flatten)]
	posts: Vec<SerializedRecommendation>,
}

impl FromIterator<Recommendation> for SerializedRecommendations {
	fn from_iter<RecommendationIterator: IntoIterator<Item = Recommendation>>(
		iterator: RecommendationIterator,
	) -> Self {
		let posts = iterator
			.into_iter()
			.map(|recommendation| SerializedRecommendation::from(&recommendation))
			.collect();
		Self { posts }
	}
}

#[derive(Serialize)]
struct SerializedRecommendation {
	title: String,
	recommendations: Vec<SerializedPost>,
}

impl From<&Recommendation> for SerializedRecommendation {
	fn from(recommendation: &Recommendation) -> Self {
		let recommendations = recommendation
			.recommended_posts()
			.iter()
			.map(SerializedPost::from)
			.collect();
		Self {
			title: recommendation.post.title().text.clone(),
			recommendations,
		}
	}
}

#[derive(Serialize)]
struct SerializedPost {
	title: String,
}

impl From<&Post> for SerializedPost {
	fn from(post: &Post) -> Self {
		Self {
			title: post.title().text.clone(),
		}
	}
}
