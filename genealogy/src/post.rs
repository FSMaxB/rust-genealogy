use crate::post::description::Description;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use chrono::NaiveDate;
use std::collections::BTreeSet;

mod description;
mod slug;
mod tag;
mod talk;
mod title;
mod video_slug;

pub trait PostTrait {
	fn title(&self) -> &Title;
	fn tags(&self) -> &BTreeSet<Tag>;
	fn date(&self) -> NaiveDate;
	fn description(&self) -> &Description;
	fn slug(&self) -> &Slug;
}
