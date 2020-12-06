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

mod article;
mod content;
mod description;
mod repository;
mod slug;
mod tag;
mod talk;
mod title;
mod video;
mod video_slug;

pub trait PostTrait {
	fn title(&self) -> &Title;
	fn tags(&self) -> &BTreeSet<Tag>;
	fn date(&self) -> NaiveDate;
	fn description(&self) -> &Description;
	fn slug(&self) -> &Slug;
}

#[allow(dead_code)]
pub enum Post {
	Article(Article),
	Talk(Talk),
	Video(Video),
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
