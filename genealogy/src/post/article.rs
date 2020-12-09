use crate::post::content::Content;
use crate::post::description::Description;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::PostTrait;
use chrono::NaiveDate;
use debug_stub_derive::DebugStub;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(DebugStub)]
pub struct Article {
	pub title: Title,
	pub tags: BTreeSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub repository: Option<Repository>,
	#[debug_stub = "Content"]
	pub content: Content,
}

impl PostTrait for Article {
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

// RUSTIFICATION: Implement PartialEq etc. for ALL Posts in one single impl block.
impl PartialEq for Article {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

impl Eq for Article {}

// NOTE: Not part of the original, but very helpful.
impl PartialOrd for Article {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(&other))
	}
}

// NOTE: Not part of the original, but very helpful.
impl Ord for Article {
	fn cmp(&self, other: &Self) -> Ordering {
		self.slug.cmp(&other.slug)
	}
}

impl Hash for Article {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
