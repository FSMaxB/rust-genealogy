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

/// ```java
/// public final class VideoFactory {
///
/// 	private VideoFactory() {
/// 		// private constructor to prevent accidental instantiation of utility class
/// 	}
/// ```
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub enum VideoFactory {}

impl VideoFactory {
	/// ```java
	/// public static Video createVideo(Path file) {
	///		try {
	///			RawPost post = PostFactory.readPost(file);
	///			return createVideo(post);
	///		} catch (RuntimeException ex) {
	///			throw new RuntimeException("Creating video failed: " + file, ex);
	///		}
	///	}
	/// ```
	pub fn create_video(file: Path) -> Result<Video, Exception> {
		// simulate try-catch
		(|| {
			let post = PostFactory::read_post_from_path(file.clone())?;
			Self::create_video_from_raw_post(post)
		})()
		.map_err(|ex| RuntimeException(r#"Creating video failed: ""# + file, ex.into()))
	}

	/// ```java
	/// private static Video createVideo(RawPost post) {
	///		RawFrontMatter frontMatter = post.frontMatter();
	///		return new Video(
	///				new Title(frontMatter.requiredValueOf(TITLE)),
	///				Tag.from(frontMatter.requiredValueOf(TAGS)),
	///				LocalDate.parse(frontMatter.requiredValueOf(DATE)),
	///				new Description(frontMatter.requiredValueOf(DESCRIPTION)),
	///				new Slug(frontMatter.requiredValueOf(SLUG)),
	///				new VideoSlug(frontMatter.requiredValueOf(VIDEO)),
	///				frontMatter.valueOf(REPOSITORY).map(Repository::new));
	///	}
	/// ```
	fn create_video_from_raw_post(post: RawPost) -> Result<Video, Exception> {
		let front_matter = post.front_matter();
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
