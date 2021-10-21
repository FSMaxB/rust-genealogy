use crate::helpers::set::Set;
use crate::helpers::time::LocalDate;
use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use crate::post::Post;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use url::Url;

/// ```java
/// public record Talk(
///		Title title,
///		Set<Tag> tags,
///		LocalDate date,
///		Description description,
///		Slug slug,
///		URI slides,
///		Optional<VideoSlug> video) implements Post {
/// ```
///
/// The `implements Post` can't be emulated directly since there is no
/// inheritance in rust and traits cannot be `sealed`. Therefore [`Post`]
/// is an enum instead and the `implements` is emulated by a [`From`] implementation.
#[derive(Debug)]
pub struct Talk {
	pub title: Title,
	tags: HashSet<Tag>,
	pub date: LocalDate,
	pub description: Description,
	pub slug: Slug,
	pub slides: Url,
	pub video: Option<VideoSlug>,
}

impl Talk {
	/// ```java
	/// 	public Talk {
	///		requireNonNull(title);
	///		requireNonNull(tags);
	///		requireNonNull(date);
	///		requireNonNull(description);
	///		requireNonNull(slug);
	///		requireNonNull(slides);
	///		requireNonNull(video);
	///	}
	/// ```
	pub fn new(
		title: Title,
		tags: HashSet<Tag>,
		date: LocalDate,
		description: Description,
		slug: Slug,
		slides: Url,
		video: Option<VideoSlug>,
	) -> Self {
		Self {
			title,
			tags,
			date,
			description,
			slug,
			slides,
			video,
		}
	}

	/// ```java
	/// @Override
	///	public Set<Tag> tags() {
	///		return Set.copyOf(tags);
	///	}
	/// ```
	pub fn tags(&self) -> HashSet<Tag> {
		Set::copy_of(&self.tags)
	}
}

/// ```java
/// public record Talk(...) implements Post
/// ```
impl From<Talk> for Post {
	fn from(talk: Talk) -> Self {
		Post::Talk(talk)
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
