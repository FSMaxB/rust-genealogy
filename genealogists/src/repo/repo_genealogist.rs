use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::genealogy::score::Score;
use genealogy::helpers::exception::Exception;
use genealogy::post::repository::Repository;
use genealogy::post::Post;
use std::convert::TryInto;
use std::sync::Arc;

pub struct RepoGenealogist;

impl Genealogist for RepoGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		let score = determine_score(&post1, &post2);
		Ok(TypedRelation {
			post1,
			post2,
			relation_type: RelationType::from_value("repo".to_string())?,
			score,
		})
	}
}

fn determine_score(post1: &Post, post2: &Post) -> Score {
	let repo1 = get_repository(post1);
	let repo2 = get_repository(post2);

	match (repo1, repo2) {
		(Some(_), None) | (None, Some(_)) => 0,
		(None, None) => 20,
		(Some(repo1), Some(repo2)) => {
			if repo1 == repo2 {
				100
			} else {
				50
			}
		}
	}
	.try_into()
	.unwrap() // We know that the number is below 100, so it is safe to unwrap
}

fn get_repository(post: &Post) -> Option<&Repository> {
	use Post::*;
	match post {
		Article(article) => article.repository.as_ref(),
		Talk(_) => None,
		Video(video) => video.repository.as_ref(),
	}
}
