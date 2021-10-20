use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video::Video;
use chrono::NaiveDate;
use std::collections::HashSet;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Post {
	Article(Article),
	Talk(Talk),
	Video(Video),
}

impl Post {
	pub fn title(&self) -> &Title {
		use Post::*;
		match self {
			Article(article) => &article.title,
			Talk(talk) => &talk.title,
			Video(video) => &video.title,
		}
	}

	pub fn tags(&self) -> &HashSet<Tag> {
		use Post::*;
		match self {
			Article(article) => &article.tags,
			Talk(talk) => &talk.tags,
			Video(video) => &video.tags,
		}
	}

	pub fn date(&self) -> NaiveDate {
		use Post::*;
		match self {
			Article(article) => article.date,
			Talk(talk) => talk.date,
			Video(video) => video.date,
		}
	}

	pub fn description(&self) -> &Description {
		use Post::*;
		match self {
			Article(article) => &article.description,
			Talk(talk) => &talk.description,
			Video(video) => &video.description,
		}
	}

	pub fn slug(&self) -> &Slug {
		use Post::*;
		match self {
			Article(article) => &article.slug,
			Talk(talk) => &talk.slug,
			Video(video) => &video.slug,
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
			tags: Tag::from_text("[Tag]")?,
			date: chrono::offset::Local::today().naive_local(),
			description: Description::from_text("description")?,
			slug: Slug::from_value(slug.to_string())?,
			repository: None,
			content: Box::new(|| Box::new(std::iter::once("".to_string()))),
		};
		Ok(Post::Article(article))
	}
}
