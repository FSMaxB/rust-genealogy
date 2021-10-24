use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::GenealogistTrait;
use genealogy::helpers::exception::Exception;
use genealogy::helpers::optional::Optional;
use genealogy::post::repository::Repository;
use genealogy::post::Post;

pub struct RepoGenealogist;

impl GenealogistTrait for RepoGenealogist {
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		let score = determine_score(&post1, &post2);
		TypedRelation::new(post1, post2, RelationType::new("repo".into())?, score)
	}
}

fn determine_score(post1: &Post, post2: &Post) -> i64 {
	let repo1 = get_repository(post1);
	let repo2 = get_repository(post2);

	match (repo1.get(), repo2.get()) {
		(Ok(_), Err(_)) | (Err(_), Ok(_)) => 0,
		(Err(_), Err(_)) => 20,
		(Ok(repo1), Ok(repo2)) => {
			if repo1 == repo2 {
				100
			} else {
				50
			}
		}
	}
}

fn get_repository(post: &Post) -> Optional<&Repository> {
	use Post::*;
	match post {
		Article(article) => article.repository.as_ref(),
		Talk(_) => Optional::empty(),
		Video(video) => video.repository.as_ref(),
	}
}
