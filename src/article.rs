use crate::article::content::Content;
use crate::article::description::Description;
use crate::article::slug::Slug;
use crate::article::tag::Tag;
use crate::article::title::Title;
use chrono::NaiveDate;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

mod article_factory;
mod content;
mod description;
mod slug;
mod tag;
mod title;

#[derive(Debug)]
pub struct Article {
	pub title: Title,
	pub tags: HashSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub content: Content,
}

impl PartialEq for Article {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

impl Eq for Article {}

impl Hash for Article {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}
