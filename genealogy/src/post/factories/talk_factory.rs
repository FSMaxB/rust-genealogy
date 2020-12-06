use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgument, RuntimeException};
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::{PostFactory, DATE, DESCRIPTION, SLIDES, SLUG, TAGS, TITLE, VIDEO};
use crate::post::factories::raw_post::RawPost;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use std::path::PathBuf;
use url::Url;

pub struct TalkFactory;

impl TalkFactory {
	#[allow(dead_code)]
	pub fn create_talk(file: &PathBuf) -> Result<Talk, Exception> {
		let post = PostFactory::read_post_from_path(file).map_err(|error| {
			RuntimeException(format!(
				r#"Creating talk failed: "{}", error: {}"#,
				file.to_string_lossy(),
				error
			))
		})?;
		create_talk(post)
	}
}

fn create_talk(post: RawPost) -> Result<Talk, Exception> {
	let front_matter = post.front_matter;
	Ok(Talk {
		title: Title::from_text(front_matter.value_of(TITLE)?)?,
		tags: Tag::from_text(front_matter.value_of(TAGS)?),
		date: parse_date(front_matter.value_of(DATE)?)?,
		description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
		slug: Slug::from_value(front_matter.value_of(SLUG)?.to_string())?,
		slides: Url::parse(front_matter.value_of(SLIDES)?).map_err(|error| IllegalArgument(error.to_string()))?,
		video: front_matter
			.value_of(VIDEO)
			.ok()
			.map(str::to_string)
			.map(VideoSlug::from_value)
			.transpose()?,
	})
}
