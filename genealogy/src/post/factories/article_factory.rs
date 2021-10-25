use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::helpers::list::List;
use crate::helpers::path::Path;
use crate::helpers::string::JString;
use crate::helpers::time::{LocalDate, LocalDateExtension};
use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::factories::post_factory::PostFactory;
use crate::post::factories::raw_post::RawPost;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;

/// ```java
/// public final class ArticleFactory {
///
/// 	private ArticleFactory() {
/// 		// private constructor to prevent accidental instantiation of utility class
/// 	}
/// ```
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub enum ArticleFactory {}

impl ArticleFactory {
	/// ```java
	/// public static Article createArticle(Path file) {
	///		try {
	///			RawPost post = PostFactory.readPost(file);
	///			return createArticle(post);
	///		} catch (RuntimeException ex) {
	///			throw new RuntimeException("Creating article failed: " + file, ex);
	///		}
	///	}
	/// ```
	pub fn create_article(file: Path) -> Result<Article, Exception> {
		// simulate try catch
		(|| {
			let post = PostFactory::read_post_from_path(file.clone())?;
			Self::create_article_from_raw_post(post)
		})()
		.map_err(|ex| RuntimeException("Creating article failed: " + file, ex.into()))
	}

	/// ```java
	/// public static Article createArticle(List<String> fileLines) {
	///		RawPost post = PostFactory.readPost(fileLines);
	///		return createArticle(post);
	///	}
	/// ```
	/// Note: The method has been renamed because rust doesn't have any overloading.
	pub fn create_article_from_lines(file_lines: List<JString>) -> Result<Article, Exception> {
		let post = PostFactory::read_post(file_lines)?;
		Self::create_article_from_raw_post(post)
	}

	/// ```java
	/// private static Article createArticle(RawPost post) {
	///		RawFrontMatter frontMatter = post.frontMatter();
	///		return new Article(
	///				new Title(frontMatter.requiredValueOf(TITLE)),
	///				Tag.from(frontMatter.requiredValueOf(TAGS)),
	///				LocalDate.parse(frontMatter.requiredValueOf(DATE)),
	///				new Description(frontMatter.requiredValueOf(DESCRIPTION)),
	///				new Slug(frontMatter.requiredValueOf(SLUG)),
	///				frontMatter.valueOf(REPOSITORY).map(Repository::new),
	///				post.content());
	///	}
	/// ```
	/// Note: The method has been renamed because rust doesn't have any overloading.
	fn create_article_from_raw_post(post: RawPost) -> Result<Article, Exception> {
		let front_matter = post.front_matter();
		Ok(Article::new(
			Title::new(front_matter.required_value_of(PostFactory::TITLE())?)?,
			Tag::from(front_matter.required_value_of(PostFactory::TAGS())?)?,
			LocalDate::parse(front_matter.required_value_of(PostFactory::DATE())?)?,
			Description::new(front_matter.required_value_of(PostFactory::DESCRIPTION())?)?,
			Slug::new(front_matter.required_value_of(PostFactory::SLUG())?)?,
			front_matter.value_of(PostFactory::REPOSITORY()).map(Repository::new)?,
			post.content(),
		))
	}
}

/// ```java
/// class ArticleFactoryTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;
	use crate::helpers::time::LocalDate;
	use crate::helpers::time::LocalDateExtension;
	use crate::post::content::ContentExtensions;

	/// ```java
	/// @Test
	///	void createFromFrontMatter_multipleColons_getValidArticle() {
	///		var file = List.of(
	///				"---",
	///				"title: Cool: A blog post",
	///				"tags: [$TAG, $TOG]",
	///				"date: 2020-01-23",
	///				"description: \"Very blog, much post, so wow\"",
	///				"slug: cool-blog-post",
	///				"---",
	///				""
	///		);
	///
	///		var post = ArticleFactory.createArticle(file);
	///
	///		assertThat(post.title().text()).isEqualTo("Cool: A blog post");
	///		assertThat(post.tags()).extracting(Tag::text).containsExactlyInAnyOrder("$TAG", "$TOG");
	///		assertThat(post.date()).isEqualTo(LocalDate.of(2020, 1, 23));
	///		assertThat(post.description().text()).isEqualTo("Very blog, much post, so wow");
	///		assertThat(post.slug().value()).isEqualTo("cool-blog-post");
	///	}
	/// ```
	#[test]
	pub(super) fn create_from_front_matter__multiple_colons__get_valid_article() {
		let file = List::of([
			"---".into(),
			"title: Cool: A blog post".into(),
			"tags: [$TAG, $TOG]".into(),
			"date: 2020-01-23".into(),
			r#"description: "Very blog, much post, so wow""#.into(),
			"slug: cool-blog-post".into(),
			"---".into(),
			"".into(),
		]);

		let post = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_that(&post.title.text).is_equal_to("Cool: A blog post");
		assert_that(post.tags())
			.extracting(Tag::text)
			.contains_exactly_in_any_order(["$TAG", "$TOG"]);
		assert_that(post.date).is_equal_to(LocalDate::of(2020, 1, 23));
		assert_that(&post.description.text).is_equal_to("Very blog, much post, so wow");
		assert_that(&post.slug.value).is_equal_to("cool-blog-post");
	}

	/// ```java
	/// @Test
	///	void createFromFrontMatter_allTagsCorrect_getValidArticle() {
	///		var file = List.of(
	///				"---",
	///				"title: A cool blog post",
	///				"tags: [$TAG, $TOG]",
	///				"date: 2020-01-23",
	///				"description: \"Very blog, much post, so wow\"",
	///				"slug: cool-blog-post",
	///				"---",
	///				""
	///		);
	///
	///		var article = ArticleFactory.createArticle(file);
	///
	///		assertThat(article.title().text()).isEqualTo("A cool blog post");
	///		assertThat(article.tags()).extracting(Tag::text).containsExactlyInAnyOrder("$TAG", "$TOG");
	///		assertThat(article.date()).isEqualTo(LocalDate.of(2020, 1, 23));
	///		assertThat(article.description().text()).isEqualTo("Very blog, much post, so wow");
	///		assertThat(article.slug().value()).isEqualTo("cool-blog-post");
	///	}
	/// ````
	#[test]
	pub(super) fn create_from_front_matter__all_tags_correct__get_valid_article() {
		let file = List::of([
			"---".into(),
			"title: A cool blog post".into(),
			"tags: [$TAG, $TOG]".into(),
			"date: 2020-01-23".into(),
			r#"description: "Very blog, much post, so wow""#.into(),
			"slug: cool-blog-post".into(),
			"---".into(),
			"".into(),
		]);

		let article = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_that(&article.title.text).is_equal_to("A cool blog post");
		assert_that(article.tags())
			.extracting(Tag::text)
			.contains_exactly_in_any_order(["$TAG", "$TOG"]);
		assert_that(article.date).is_equal_to(LocalDate::of(2020, 1, 23));
		assert_that(&article.description.text).is_equal_to("Very blog, much post, so wow");
		assert_that(&article.slug.value).is_equal_to("cool-blog-post");
	}

	/// ```java
	/// @Test
	///	void createFromFile_allTagsCorrect_getValidArticle() {
	///		var file = List.of(
	///				"---",
	///				"title: A cool blog post",
	///				"tags: [$TAG, $TOG]",
	///				"date: 2020-01-23",
	///				"description: \"Very blog, much post, so wow\"",
	///				"slug: cool-blog-post",
	///				"---",
	///				"",
	///				"Lorem ipsum dolor sit amet.",
	///				"Ut enim ad minim veniam.",
	///				"Duis aute irure dolor in reprehenderit.",
	///				"Excepteur sint occaecat cupidatat non proident.");
	///
	///		var article = ArticleFactory.createArticle(file);
	///
	///		assertThat(article.title().text()).isEqualTo("A cool blog post");
	///		assertThat(article.tags()).extracting(Tag::text).containsExactlyInAnyOrder("$TAG", "$TOG");
	///		assertThat(article.date()).isEqualTo(LocalDate.of(2020, 1, 23));
	///		assertThat(article.description().text()).isEqualTo("Very blog, much post, so wow");
	///		assertThat(article.slug().value()).isEqualTo("cool-blog-post");
	///		assertThat(article.content().get()).containsExactly(
	///				"",
	///				"Lorem ipsum dolor sit amet.",
	///				"Ut enim ad minim veniam.",
	///				"Duis aute irure dolor in reprehenderit.",
	///				"Excepteur sint occaecat cupidatat non proident.");
	///	}
	/// ```
	#[test]
	pub(super) fn creat_from_file_all_tags_correct_get_valid_article() {
		let file = List::of([
			"---".into(),
			"title: A cool blog post".into(),
			"tags: [$TAG, $TOG]".into(),
			"date: 2020-01-23".into(),
			r#"description: "Very blog, much post, so wow""#.into(),
			"slug: cool-blog-post".into(),
			"---".into(),
			"".into(),
			"Lorem ipsum dolor sit amet.".into(),
			"Ut enim ad minim veniam.".into(),
			"Duis aute irure dolor in reprehenderit.".into(),
			"Excepteur sint occaecat cupidatat non proident.".into(),
		]);

		let article = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_that(&article.title.text).is_equal_to("A cool blog post");
		assert_that(article.tags())
			.extracting(Tag::text)
			.contains_exactly_in_any_order(["$TAG", "$TOG"]);
		assert_that(article.date).is_equal_to(LocalDate::of(2020, 1, 23));
		assert_that(&article.description.text).is_equal_to("Very blog, much post, so wow");
		assert_that(&article.slug.value).is_equal_to("cool-blog-post");
		assert_that(article.content.get().to_list().unwrap()).contains_exactly([
			"",
			"Lorem ipsum dolor sit amet.",
			"Ut enim ad minim veniam.",
			"Duis aute irure dolor in reprehenderit.",
			"Excepteur sint occaecat cupidatat non proident.",
		]);
	}
}
