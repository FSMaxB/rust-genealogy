use crate::post::description::Description;
use crate::post::factories::post_factory::PostFactory;
use crate::post::factories::raw_post::RawPost;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::talk::Talk;
use crate::post::title::Title;
use crate::post::video_slug::VideoSlug;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::exception::Exception::{IllegalArgumentException, RuntimeException, URISyntaxException};
use genealogy_java_apis::path::Path;
use genealogy_java_apis::time::{LocalDate, LocalDateExtension};
use genealogy_java_apis::uri::URI;

/// ```java
/// public final class TalkFactory {
///
/// 	private TalkFactory() {
/// 		// private constructor to prevent accidental instantiation of utility class
/// 	}
/// ```
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub enum TalkFactory {}

impl TalkFactory {
	/// ```java
	/// public static Talk createTalk(Path file) {
	///		try {
	///			RawPost post = PostFactory.readPost(file);
	///			return createTalk(post);
	///		} catch (RuntimeException ex) {
	///			throw new RuntimeException("Creating talk failed: " + file, ex);
	///		}
	///	}
	/// ```
	pub fn create_talk(file: Path) -> Result<Talk, Exception> {
		// simulate try-catch
		(|| {
			let post = PostFactory::read_post_from_path(file.clone())?;
			Self::create_talk_from_raw_post(post)
		})()
		.map_err(|ex| RuntimeException("Creating talk failed: " + file, ex.into()))
	}

	/// ```java
	/// private static Talk createTalk(RawPost post) {
	///		RawFrontMatter frontMatter = post.frontMatter();
	///		try {
	///			return new Talk(
	///					new Title(frontMatter.requiredValueOf(TITLE)),
	///					Tag.from(frontMatter.requiredValueOf(TAGS)),
	///					LocalDate.parse(frontMatter.requiredValueOf(DATE)),
	///					new Description(frontMatter.requiredValueOf(DESCRIPTION)),
	///					new Slug(frontMatter.requiredValueOf(SLUG)),
	///					new URI(frontMatter.requiredValueOf(SLIDES)),
	///					frontMatter.valueOf(VIDEO).map(VideoSlug::new));
	///		} catch (URISyntaxException ex) {
	///			throw new IllegalArgumentException(ex);
	///		}
	///	}
	/// ```
	fn create_talk_from_raw_post(post: RawPost) -> Result<Talk, Exception> {
		let front_matter = post.front_matter();
		// simulate try-catch
		(|| {
			Ok(Talk::new(
				Title::new(front_matter.required_value_of(PostFactory::TITLE())?)?,
				Tag::from(front_matter.required_value_of(PostFactory::TAGS())?)?,
				LocalDate::parse(front_matter.required_value_of(PostFactory::DATE())?)?,
				Description::new(front_matter.required_value_of(PostFactory::DESCRIPTION())?)?,
				Slug::new(front_matter.required_value_of(PostFactory::SLUG())?)?,
				URI::new(front_matter.required_value_of(PostFactory::SLIDES())?)?,
				front_matter.value_of(PostFactory::VIDEO()).map(VideoSlug::new)?,
			))
		})()
		.map_err(|error| match error {
			URISyntaxException(ex) => IllegalArgumentException(ex.to_string().into()),
			other => other,
		})
	}
}
