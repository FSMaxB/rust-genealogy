use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::raw_post::{RawPost, DATE, DESCRIPTION, REPOSITORY, SLUG, TAGS, TITLE, VIDEO};
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Debug)]
pub struct Video {
	pub title: Title,
	pub tags: HashSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub video: VideoSlug,
	pub repository: Option<Repository>,
}

impl PartialEq for Video {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

// NOTE: Not part of the original, but very helpful.
impl PartialOrd for Video {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// NOTE: Not part of the original, but very helpful.
impl Ord for Video {
	fn cmp(&self, other: &Self) -> Ordering {
		self.slug.cmp(&other.slug)
	}
}

impl Eq for Video {}

impl Hash for Video {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}

impl TryFrom<&Path> for Video {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		RawPost::try_from(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating video failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Video::try_from)
	}
}

impl TryFrom<RawPost> for Video {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter;
		Ok(Video {
			title: Title::from_text(front_matter.value_of(TITLE)?)?,
			tags: Tag::from_text(front_matter.value_of(TAGS)?)?,
			date: parse_date(front_matter.value_of(DATE)?)?,
			description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
			slug: Slug::from_value(front_matter.value_of(SLUG)?.to_string())?,
			video: VideoSlug::from_value(front_matter.value_of(VIDEO)?.to_string())?,
			repository: front_matter
				.value_of(REPOSITORY)
				.ok()
				.map(str::to_string)
				.map(Repository::from_identifier)
				.transpose()?,
		})
	}
}
