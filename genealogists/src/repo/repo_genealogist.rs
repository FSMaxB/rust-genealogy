use genealogy::genealogist::relation_type::RelationType;
use genealogy::genealogist::typed_relation::TypedRelation;
use genealogy::genealogist::GenealogistTrait;
use genealogy::post::repository::Repository;
use genealogy::post::Post;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::objects::Objects;
use genealogy_java_apis::optional::Optional;
use genealogy_java_apis::r#static;
use std::fmt::{Display, Formatter};

/// ```java
/// public class RepoGenealogist implements Genealogist {
/// ```
#[derive(Debug)]
pub struct RepoGenealogist;

/// ```java
/// public class RepoGenealogist implements Genealogist {
/// ```
/// Interface implementation
impl GenealogistTrait for RepoGenealogist {
	/// ```java
	/// @Override
	///	public TypedRelation infer(Post post1, Post post2) {
	///		long score = determineScore(post1, post2);
	///		return new TypedRelation(post1, post2, TYPE, score);
	///	}
	/// ```
	fn infer(&self, post1: Post, post2: Post) -> Result<TypedRelation, Exception> {
		let score = self.determine_score(post1.clone(), post2.clone());
		TypedRelation::new(post1, post2, Self::TYPE(), score)
	}
}

impl RepoGenealogist {
	// ```java
	// private static final RelationType TYPE = new RelationType("repo");
	// ```
	r#static!(TYPE: RelationType = RelationType::new("repo".into()).unwrap());

	/// ```java
	/// public class RepoGenealogist implements Genealogist {
	/// ```
	pub fn new() -> Self {
		Self
	}

	/// ```java
	/// private long determineScore(Post post1, Post post2) {
	///		var repo1 = getRepository(post1);
	///		var repo2 = getRepository(post2);
	///
	///		if (repo1.isPresent() != repo2.isPresent())
	///			return 0;
	///		// at this point, either both are empty or both are non-empty
	///		if (repo1.isEmpty())
	///			return 20;
	///		return Objects.equals(repo1, repo2) ? 100 : 50;
	///	}
	/// ```
	fn determine_score(&self, post1: Post, post2: Post) -> i64 {
		let repo1 = self.get_repository(post1);
		let repo2 = self.get_repository(post2);

		if repo1.is_present() != repo2.is_present() {
			return 0;
		}
		// at this point either both are empty or both are non-empty
		if repo1.is_empty() {
			return 20;
		}

		if Objects::equals(repo1, repo2) {
			100
		} else {
			50
		}
	}

	/// ```java
	/// private Optional<Repository> getRepository(Post post) {
	///		return switch (post) {
	///			case Article article -> article.repository();
	///			case Video video -> video.repository();
	///			default -> Optional.empty();
	///		};
	///	}
	/// ```
	fn get_repository(&self, post: Post) -> Optional<Repository> {
		use Post::*;
		match post {
			Article(article) => article.repository(),
			Video(video) => video.repository(),
			_ => Optional::empty(),
		}
	}
}

impl Display for RepoGenealogist {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		formatter.write_str("RepoGenealogist")
	}
}
