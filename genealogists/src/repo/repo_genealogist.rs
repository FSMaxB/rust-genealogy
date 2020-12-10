use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::Genealogist;
use genealogy::helpers::exception::Exception;
use genealogy::post::repository::Repository;
use genealogy::post::Post;
use std::sync::Arc;

pub struct RepoGenealogist;

impl Genealogist for RepoGenealogist {
	fn infer(&self, post1: Arc<Post>, post2: Arc<Post>) -> Result<TypedRelation, Exception> {
		let score = determine_score(&post1, &post2);
		TypedRelation::new(post1, post2, RelationType::from_value("repo".to_string())?, score)
	}
}

fn determine_score(post1: &Post, post2: &Post) -> u64 {
	let repo1 = get_repository(post1);
	let repo2 = get_repository(post2);

	// RUSTIFICATION: Use match
	if repo1.is_some() != repo2.is_some() {
		return 0;
	}

	if repo1.is_none() {
		return 20;
	}

	if repo1 == repo2 {
		100
	} else {
		50
	}
}

fn get_repository(post: &Post) -> Option<&Repository> {
	use Post::*;
	match post {
		Article(article) => article.repository.as_ref(),
		Talk(_) => None,
		Video(video) => video.repository.as_ref(),
	}
}
