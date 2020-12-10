use crate::helpers::classes::{Class, GetClass};
use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video::Video;
use chrono::NaiveDate;
use std::collections::BTreeSet;
use std::ops::Deref;

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

pub trait PostTrait {
	fn title(&self) -> &Title;
	fn tags(&self) -> &BTreeSet<Tag>;
	fn date(&self) -> NaiveDate;
	fn description(&self) -> &Description;
	fn slug(&self) -> &Slug;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Post {
	Article(Article),
	Talk(Talk),
	Video(Video),
}

// NOTE: 😝
impl GetClass for Post {
	fn get_class(&self) -> Class {
		use Post::*;
		match self {
			Article(_) => "Article",
			Talk(_) => "Talk",
			Video(_) => "Video",
		}
		.into()
	}
}

// NOTE: Although one could manually implement `PostTrait` for `Post`,
// this is much easier to write and should work the same ergonomically when using it.
impl Deref for Post {
	type Target = dyn PostTrait;

	fn deref(&self) -> &Self::Target {
		use Post::*;
		match self {
			Article(article) => article as &dyn PostTrait,
			Talk(talk) => talk as &dyn PostTrait,
			Video(video) => video as &dyn PostTrait,
		}
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::helpers::exception::Exception;

	pub fn post_with_slug(slug: &str) -> Result<Post, Exception> {
		let article = Article {
			title: Title::from_text("title")?,
			tags: Tag::from_text("[Tag]"),
			date: chrono::offset::Local::today().naive_local(),
			description: Description::from_text("description")?,
			slug: Slug::from_value(slug.to_string())?,
			repository: None,
			content: Box::new(|| Box::new(std::iter::once("".to_string()))),
		};
		Ok(Post::Article(article))
	}
}
