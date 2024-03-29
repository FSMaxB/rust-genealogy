use crate::post::content::Content;
use crate::post::description::Description;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::Post;
use genealogy_java_apis::optional::Optional;
use genealogy_java_apis::record;
use genealogy_java_apis::set::Set;
use genealogy_java_apis::time::LocalDate;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// ```java
/// public record Article(
///		Title title,
///		Set<Tag> tags,
///		LocalDate date,
///		Description description,
///		Slug slug,
///		Optional<Repository> repository,
///		Content content) implements Post {
///
/// 	public Article {
///			requireNonNull(title);
///			requireNonNull(tags);
///			requireNonNull(date);
///			requireNonNull(description);
///			requireNonNull(slug);
///			requireNonNull(repository);
///			requireNonNull(content);
///	}
/// ```
///
/// The `implements Post` can't be emulated directly since there is no
/// inheritance in rust and traits cannot be `sealed`. Therefore [`Post`]
/// is an enum instead and the `implements` is emulated by a [`From`] implementation.
#[record(equals = false, hash = false)]
pub struct Article {
	title: Title,
	#[omit]
	tags: Set<Tag>,
	date: LocalDate,
	description: Description,
	slug: Slug,
	repository: Optional<Repository>,
	content: Content,
}

impl Article {
	/// ```java
	/// @Override
	///	public Set<Tag> tags() {
	///		return Set.copyOf(tags);
	///	}
	/// ```
	pub fn tags(&self) -> Set<Tag> {
		Set::copy_of(self.tags.clone())
	}
}

/// ```java
/// public record Article(...) implements Post
/// ```
impl From<Article> for Post {
	fn from(article: Article) -> Self {
		Post::Article(Rc::new(article))
	}
}

/// ```java
/// @Override
///	public boolean equals(Object o) {
///		if (this == o)
///			return true;
///		if (o == null || getClass() != o.getClass())
///			return false;
///		Article article = (Article) o;
///		return slug.equals(article.slug);
///	}
/// ```
impl PartialEq for Article {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

/// ```java
/// @Override
///	public boolean equals(Object o) {
///		if (this == o)
///			return true;
///		if (o == null || getClass() != o.getClass())
///			return false;
///		Article article = (Article) o;
///		return slug.equals(article.slug);
///	}
/// ```
impl Eq for Article {}

/// ```java
///	@Override
///	public int hashCode() {
///		return Objects.hash(slug);
///	}
/// ```
impl Hash for Article {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
