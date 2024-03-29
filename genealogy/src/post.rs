use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video::Video;
use genealogy_java_apis::set::Set;
use genealogy_java_apis::time::LocalDate;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod article;
pub mod content;
pub mod description;
pub mod factories;
pub mod repository;
pub mod slug;
pub mod tag;
pub mod talk;
pub mod title;
pub mod video;
pub mod video_slug;

/// ```java
/// public sealed interface Post permits Article, Talk, Video
/// ```
///
/// enum instead of sealed interface. The semantics are roughly equivalent
/// since both are sum types.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Post {
	Article(Rc<Article>),
	Talk(Rc<Talk>),
	Video(Rc<Video>),
}

use Post::*;

impl Post {
	/// ```java
	/// Title title();
	/// ```
	pub fn title(&self) -> Title {
		match self {
			Article(article) => article.title(),
			Talk(talk) => talk.title(),
			Video(video) => video.title(),
		}
	}

	/// ```java
	/// Set<Tag> tags();
	/// ```
	pub fn tags(&self) -> Set<Tag> {
		match self {
			Article(article) => article.tags(),
			Talk(talk) => talk.tags(),
			Video(video) => video.tags(),
		}
	}

	/// ```java
	/// LocalDate date();
	/// ```
	pub fn date(&self) -> LocalDate {
		match self {
			Article(article) => article.date(),
			Talk(talk) => talk.date(),
			Video(video) => video.date(),
		}
	}

	/// ```java
	/// Description description();
	/// ```
	pub fn description(&self) -> Description {
		match self {
			Article(article) => article.description(),
			Talk(talk) => talk.description(),
			Video(video) => video.description(),
		}
	}

	/// ```java
	/// Slug slug();
	/// ```
	pub fn slug(&self) -> Slug {
		match self {
			Article(article) => article.slug(),
			Talk(talk) => talk.slug(),
			Video(video) => video.slug(),
		}
	}
}

impl Display for Post {
	fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
		match self {
			Article(article) => article.fmt(formatter),
			Talk(talk) => talk.fmt(formatter),
			Video(video) => video.fmt(formatter),
		}
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use genealogy_java_apis::exception::Exception;
	use genealogy_java_apis::optional::Optional;
	use genealogy_java_apis::stream::Stream;
	use genealogy_java_apis::string::JString;
	use genealogy_java_apis::time::LocalDateExtension;

	/// ```java
	/// public class PostTestHelper {
	/// ```
	pub struct PostTestHelper;

	impl PostTestHelper {
		/// ```java
		/// public static Post createWithSlug(String slug) {
		///		return new Article(
		///				new Title("Title"),
		///				Tag.from("[Tag]"),
		///				LocalDate.now(),
		///				new Description("description"),
		///				new Slug(slug),
		///				Optional.empty(),
		///				() -> Stream.of(""));
		///	}
		/// ```
		pub fn create_with_slug(slug: JString) -> Result<Post, Exception> {
			Ok(Article::new(
				Title::new("Title".into())?,
				Tag::from("[Tag]".into())?,
				LocalDate::today(),
				Description::new("description".into())?,
				Slug::new(slug)?,
				Optional::empty(),
				(|| Stream::of(["".into()])).into(),
			)
			.into())
		}
	}
}
