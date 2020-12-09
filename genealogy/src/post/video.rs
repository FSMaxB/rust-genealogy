use crate::post::description::Description;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use crate::post::PostTrait;
use chrono::NaiveDate;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Video {
	pub title: Title,
	pub tags: BTreeSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub video: VideoSlug,
	pub repository: Option<Repository>,
}

impl PostTrait for Video {
	fn title(&self) -> &Title {
		&self.title
	}

	fn tags(&self) -> &BTreeSet<Tag> {
		&self.tags
	}

	fn date(&self) -> NaiveDate {
		self.date
	}

	fn description(&self) -> &Description {
		&self.description
	}

	fn slug(&self) -> &Slug {
		&self.slug
	}
}

impl PartialEq for Video {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

// NOTE: Not part of the original, but very helpful.
impl PartialOrd for Video {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
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
