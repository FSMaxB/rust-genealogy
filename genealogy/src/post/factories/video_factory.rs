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
use std::path::Path;

impl TryFrom<&Path> for Video {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		RawPost::try_from(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating video failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Video::try_from)
	}
}

impl TryFrom<RawPost> for Video {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter;
		Ok(Video::new(
			Title::new(front_matter.value_of(TITLE)?)?,
			Tag::from(front_matter.value_of(TAGS)?)?,
			parse_date(front_matter.value_of(DATE)?)?,
			Description::new(front_matter.value_of(DESCRIPTION)?)?,
			Slug::new(front_matter.value_of(SLUG)?.to_string())?,
			VideoSlug::new(front_matter.value_of(VIDEO)?.to_string())?,
			front_matter
				.value_of(REPOSITORY)
				.ok()
				.map(str::to_string)
				.map(Repository::new)
				.transpose()?,
		))
	}
}
