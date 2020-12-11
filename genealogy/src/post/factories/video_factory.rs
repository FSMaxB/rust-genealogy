use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::raw_post::{RawPost, DATE, DESCRIPTION, REPOSITORY, SLUG, TAGS, TITLE, VIDEO};
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video::Video;
use crate::post::video_slug::VideoSlug;
use std::convert::TryFrom;
use std::path::Path;

pub struct VideoFactory;

impl VideoFactory {
	pub fn create_video(path: &Path) -> Result<Video, Exception> {
		let post = RawPost::try_from(path).map_err(|error| {
			RuntimeException(format!(
				r#"Creating video failed: "{}", error: {}"#,
				path.to_string_lossy(),
				error
			))
		})?;
		create_video(post)
	}
}

fn create_video(post: RawPost) -> Result<Video, Exception> {
	let front_matter = post.front_matter;
	Ok(Video {
		title: Title::from_text(front_matter.value_of(TITLE)?)?,
		tags: Tag::from_text(front_matter.value_of(TAGS)?),
		date: parse_date(front_matter.value_of(DATE)?)?,
		description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
		slug: Slug::from_value(front_matter.value_of(SLUG)?.to_string())?,
		video: VideoSlug::from_value(front_matter.value_of(VIDEO)?.to_string())?,
		repository: front_matter
			.value_of(REPOSITORY)
			.ok()
			.map(str::to_string)
			.map(Repository::from_identifier)
			.transpose()?,
	})
}
