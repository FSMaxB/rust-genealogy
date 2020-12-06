use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use crate::post::PostTrait;
use chrono::NaiveDate;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
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

impl PostTrait for Talk {
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

impl PartialEq for Talk {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

impl Eq for Talk {}

impl Hash for Talk {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
