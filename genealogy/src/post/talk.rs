use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgumentException, RuntimeException};
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::raw_post::{RawPost, DATE, DESCRIPTION, SLIDES, SLUG, TAGS, TITLE, VIDEO};
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::path::Path;
use url::Url;

#[derive(Debug)]
pub struct Talk {
	pub title: Title,
	pub tags: BTreeSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub slides: Url,
	pub video: Option<VideoSlug>,
}

impl PartialEq for Talk {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

impl Eq for Talk {}

// NOTE: Not part of the original, but very helpful.
impl PartialOrd for Talk {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.slug.cmp(&other.slug))
	}
}

// NOTE: Not part of the original, but very helpful.
impl Ord for Talk {
	fn cmp(&self, other: &Self) -> Ordering {
		self.slug.cmp(&other.slug)
	}
}

impl Hash for Talk {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}

impl TryFrom<&Path> for Talk {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		RawPost::try_from(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating talk failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Talk::try_from)
	}
}

impl TryFrom<RawPost> for Talk {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter;
		Ok(Talk {
			title: Title::from_text(front_matter.value_of(TITLE)?)?,
			tags: Tag::from_text(front_matter.value_of(TAGS)?),
			date: parse_date(front_matter.value_of(DATE)?)?,
			description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
			slug: Slug::from_value(front_matter.value_of(SLUG)?.to_string())?,
			slides: Url::parse(front_matter.value_of(SLIDES)?)
				.map_err(|error| IllegalArgumentException(error.to_string()))?,
			video: front_matter
				.value_of(VIDEO)
				.ok()
				.map(str::to_string)
				.map(VideoSlug::from_value)
				.transpose()?,
		})
	}
}
