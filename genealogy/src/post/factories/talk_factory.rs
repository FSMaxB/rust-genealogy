use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgumentException, RuntimeException};
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::{DATE, DESCRIPTION, SLIDES, SLUG, TAGS, TITLE, VIDEO};
use crate::post::factories::raw_post::RawPost;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use std::path::Path;
use url::Url;

impl TryFrom<&Path> for Talk {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		RawPost::try_from(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating talk failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Talk::try_from)
	}
}

impl TryFrom<RawPost> for Talk {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter();
		Ok(Talk::new(
			Title::new(&front_matter.required_value_of(TITLE)?)?,
			Tag::from(&front_matter.required_value_of(TAGS)?)?,
			parse_date(&front_matter.required_value_of(DATE)?)?,
			Description::new(&front_matter.required_value_of(DESCRIPTION)?)?,
			Slug::new(front_matter.required_value_of(SLUG)?)?,
			Url::parse(&front_matter.required_value_of(SLIDES)?)
				.map_err(|error| IllegalArgumentException(error.to_string()))?,
			front_matter
				.required_value_of(VIDEO)
				.ok()
				.map(VideoSlug::new)
				.transpose()?,
		))
	}
}
