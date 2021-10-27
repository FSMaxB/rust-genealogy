use crate::post::description::Description;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use crate::post::Post;
use genealogy_java_apis::optional::Optional;
use genealogy_java_apis::record;
use genealogy_java_apis::set::Set;
use genealogy_java_apis::time::LocalDate;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// ```java
/// public record Video(
///		Title title,
///		Set<Tag> tags,
///		LocalDate date,
///		Description description,
///		Slug slug,
///		VideoSlug video,
///		Optional<Repository> repository) implements Post {
///
/// 	public Video {
///			requireNonNull(title);
///			requireNonNull(tags);
///			requireNonNull(date);
///			requireNonNull(description);
///			requireNonNull(slug);
///			requireNonNull(video);
///			requireNonNull(repository);
///		}
/// ```
///
/// The `implements Post` can't be emulated directly since there is no
/// inheritance in rust and traits cannot be `sealed`. Therefore [`Post`]
/// is an enum instead and the `implements` is emulated by a [`From`] implementation.
#[record]
#[derive(Debug)]
pub struct Video {
	title: Title,
	#[omit]
	tags: Set<Tag>,
	date: LocalDate,
	description: Description,
	slug: Slug,
	video: VideoSlug,
	repository: Optional<Repository>,
}

impl Video {
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
/// public record Video(...) implements Post
/// ```
impl From<Video> for Post {
	fn from(video: Video) -> Self {
		Post::Video(Rc::new(video))
	}
}

/// ```java
/// @Override
///	public boolean equals(Object o) {
///		if (this == o)
///			return true;
///		if (o == null || getClass() != o.getClass())
///			return false;
///		Video video = (Video) o;
///		return slug.equals(video.slug);
///	}
/// ```
impl PartialEq for Video {
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
///		Video video = (Video) o;
///		return slug.equals(video.slug);
///	}
/// ```
impl Eq for Video {}

/// ```java
/// @Override
///	public int hashCode() {
///		return Objects.hash(slug);
///	}
/// ```
impl Hash for Video {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
