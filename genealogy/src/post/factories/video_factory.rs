use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::PostFactory;
use crate::post::factories::raw_post::RawPost;
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
		PostFactory::read_post_from_path(path)
			.map_err(|error| RuntimeException(format!(r#"Creating video failed: "{:?}""#, path), error.into()))
			.and_then(Video::try_from)
	}
}

impl TryFrom<RawPost> for Video {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter();
		Ok(Video::new(
			Title::new(&front_matter.required_value_of(PostFactory::TITLE)?)?,
			Tag::from(&front_matter.required_value_of(PostFactory::TAGS)?)?,
			parse_date(&front_matter.required_value_of(PostFactory::DATE)?)?,
			Description::new(&front_matter.required_value_of(PostFactory::DESCRIPTION)?)?,
			Slug::new(front_matter.required_value_of(PostFactory::SLUG)?)?,
			VideoSlug::new(front_matter.required_value_of(PostFactory::VIDEO)?)?,
			front_matter
				.required_value_of(PostFactory::REPOSITORY)
				.ok()
				.map(Repository::new)
				.transpose()?,
		))
	}
}
