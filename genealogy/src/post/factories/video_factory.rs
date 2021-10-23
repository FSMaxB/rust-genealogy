use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::helpers::path::Path;
use crate::helpers::time::{LocalDate, LocalDateExtension};
use crate::post::description::Description;
use crate::post::factories::post_factory::PostFactory;
use crate::post::factories::raw_post::RawPost;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use crate::post::video::Video;
use crate::post::video_slug::VideoSlug;

impl TryFrom<Path> for Video {
	type Error = Exception;

	fn try_from(path: Path) -> Result<Self, Self::Error> {
		PostFactory::read_post_from_path(path.clone())
			.map_err(|error| RuntimeException(r#"Creating video failed: ""# + path + r#"""#, error.into()))
			.and_then(Video::try_from)
	}
}

impl TryFrom<RawPost> for Video {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter();
		Ok(Video::new(
			Title::new(front_matter.required_value_of(PostFactory::TITLE())?)?,
			Tag::from(front_matter.required_value_of(PostFactory::TAGS())?)?,
			LocalDate::parse(front_matter.required_value_of(PostFactory::DATE())?)?,
			Description::new(front_matter.required_value_of(PostFactory::DESCRIPTION())?)?,
			Slug::new(front_matter.required_value_of(PostFactory::SLUG())?)?,
			VideoSlug::new(front_matter.required_value_of(PostFactory::VIDEO())?)?,
			front_matter.value_of(PostFactory::REPOSITORY()).map(Repository::new)?,
		))
	}
}
