use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::helpers::list::List;
use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::PostFactory;
use crate::post::factories::raw_post::RawPost;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use std::path::Path;

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
	pub fn create_article(file: &Path) -> Result<Article, Exception> {
		// simulate try catch
		(|| {
			let post = PostFactory::read_post_from_path(file)?;
			Self::create_article_from_raw_post(post)
		})()
		.map_err(|ex| RuntimeException(format!("Creating article failed: {:?}", file), ex.into()))
	}

	/// ```java
	/// public static Article createArticle(List<String> fileLines) {
	///		RawPost post = PostFactory.readPost(fileLines);
	///		return createArticle(post);
	///	}
	/// ```
	/// Note: The method has been renamed because rust doesn't have any overloading.
	pub fn create_article_from_lines(file_lines: List<String>) -> Result<Article, Exception> {
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
			Title::new(front_matter.required_value_of(PostFactory::TITLE)?)?,
			Tag::from(front_matter.required_value_of(PostFactory::TAGS)?)?,
			parse_date(front_matter.required_value_of(PostFactory::DATE)?)?,
			Description::new(front_matter.required_value_of(PostFactory::DESCRIPTION)?)?,
			Slug::new(front_matter.required_value_of(PostFactory::SLUG)?.into())?,
			front_matter
				.value_of(PostFactory::REPOSITORY)
				.map(ToString::to_string)
				.map(Repository::new)
				.transpose()?,
			post.content(),
		))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::time::LocalDate;
	use crate::test_helpers::hash_set_of_tags;

	#[test]
	fn create_from_front_matter_multiple_colons_get_valid_article() {
		let file = lines(&[
			"---",
			"title: Cool: A blog post",
			"tags: [$TAG, $TOG]",
			"date: 2020-01-23",
			"description: \"Very blog, much post, so wow\"",
			"slug: cool-blog-post",
			"---",
			"",
		]);

		let post = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_eq!("Cool: A blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
	}

	#[test]
	fn create_from_front_matter_all_tags_correct_get_valid_article() {
		let file = lines(&[
			"---",
			"title: A cool blog post",
			"tags: [$TAG, $TOG]",
			"date: 2020-01-23",
			"description: \"Very blog, much post, so wow\"",
			"slug: cool-blog-post",
			"---",
			"",
		]);

		let post = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_eq!("A cool blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
	}

	#[test]
	fn creat_from_file_all_tags_correct_get_valid_article() {
		let file = lines(&[
			"---",
			"title: A cool blog post",
			"tags: [$TAG, $TOG]",
			"date: 2020-01-23",
			"description: \"Very blog, much post, so wow\"",
			"slug: cool-blog-post",
			"---",
			"",
			"Lorem ipsum dolor sit amet.",
			"Ut enim ad minim veniam.",
			"Duis aute irure dolor in reprehenderit.",
			"Excepteur sint occaecat cupidatat non proident.",
		]);

		let post = ArticleFactory::create_article_from_lines(file).unwrap();

		assert_eq!("A cool blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
		let content = (post.content)().to_list().unwrap();
		let expected_content = lines(&[
			"",
			"Lorem ipsum dolor sit amet.",
			"Ut enim ad minim veniam.",
			"Duis aute irure dolor in reprehenderit.",
			"Excepteur sint occaecat cupidatat non proident.",
		]);
		assert_eq!(expected_content, content);
	}

	fn lines(lines: &[&str]) -> List<String> {
		lines.iter().copied().map(str::to_string).collect()
	}
}
