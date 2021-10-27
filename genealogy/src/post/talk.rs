use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use crate::post::Post;
use genealogy_java_apis::optional::Optional;
use genealogy_java_apis::record;
use genealogy_java_apis::set::Set;
use genealogy_java_apis::time::LocalDate;
use genealogy_java_apis::uri::URI;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

/// ```java
/// public record Talk(
///		Title title,
///		Set<Tag> tags,
///		LocalDate date,
///		Description description,
///		Slug slug,
///		URI slides,
///		Optional<VideoSlug> video) implements Post {
///
/// 	public Talk {
///			requireNonNull(title);
///			requireNonNull(tags);
///			requireNonNull(date);
///			requireNonNull(description);
///			requireNonNull(slug);
///			requireNonNull(slides);
///			requireNonNull(video);
///		}
/// ```
///
/// The `implements Post` can't be emulated directly since there is no
/// inheritance in rust and traits cannot be `sealed`. Therefore [`Post`]
/// is an enum instead and the `implements` is emulated by a [`From`] implementation.
#[record(equals = false, hash = false)]
pub struct Talk {
	title: Title,
	#[omit]
	tags: Set<Tag>,
	date: LocalDate,
	description: Description,
	slug: Slug,
	slides: URI,
	video: Optional<VideoSlug>,
}

impl Talk {
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
/// public record Talk(...) implements Post
/// ```
impl From<Talk> for Post {
	fn from(talk: Talk) -> Self {
		Post::Talk(Rc::new(talk))
	}
}

/// ```java
/// @Override
///	public boolean equals(Object o) {
///		if (this == o)
///			return true;
///		if (o == null || getClass() != o.getClass())
///			return false;
///		Talk talk = (Talk) o;
///		return slug.equals(talk.slug);
///	}
/// ```
impl PartialEq for Talk {
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
///		Talk talk = (Talk) o;
///		return slug.equals(talk.slug);
///	}
/// ```
impl Eq for Talk {}

/// ```java
///	@Override
///	public int hashCode() {
///		return Objects.hash(slug);
///	}
/// ```
impl Hash for Talk {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
