use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::{PostFactory, DATE, DESCRIPTION, REPOSITORY, SLUG, TAGS, TITLE};
use crate::post::factories::raw_post::RawPost;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use std::path::PathBuf;

pub struct ArticleFactory;

impl ArticleFactory {
	#[allow(dead_code)]
	pub fn create_article_from_path(file: &PathBuf) -> Result<Article, Exception> {
		let post = PostFactory::read_post_from_path(file).map_err(|error| {
			RuntimeException(format!(
				r#"Creating article failed: "{}", error: {}"#,
				file.to_string_lossy(),
				error
			))
		})?;
		create_article(post)
	}

	#[allow(dead_code)]
	pub fn create_article_from_lines(file_lines: Vec<String>) -> Result<Article, Exception> {
		let post = PostFactory::read_post_from_lines(file_lines)?;
		create_article(post)
	}
}

fn create_article(post: RawPost) -> Result<Article, Exception> {
	let front_matter = post.front_matter;
	// RUSTIFICATION: Create a trait that allows simple text parsing and
	// put the constants in it as associated const so they can be used by
	// dynamic code for lookup in the front matter.
	Ok(Article {
		title: Title::from_text(front_matter.value_of(TITLE)?)?,
		tags: Tag::from_text(front_matter.value_of(TAGS)?),
		date: parse_date(front_matter.value_of(DATE)?)?,
		description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
		slug: Slug::from_value(front_matter.value_of(SLUG)?.to_string())?,
		repository: front_matter
			.value_of(REPOSITORY)
			.ok()
			.map(str::to_string)
			.map(Repository::from_identifier)
			.transpose()?,
		content: post.content,
	})
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helpers::btree_set_of_tags;
	use chrono::NaiveDate;

	#[test]
	fn create_from_front_matter_multiple_colons_get_valid_article() {
		let file = line_vector(&[
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
		assert_eq!(btree_set_of_tags(&["$TAG", "$TOG"]), post.tags);
		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
	}

	#[test]
	fn create_from_front_matter_all_tags_correct_get_valid_article() {
		let file = line_vector(&[
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
		assert_eq!(btree_set_of_tags(&["$TAG", "$TOG"]), post.tags);
		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
	}

	#[test]
	fn creat_from_file_all_tags_correct_get_valid_article() {
		let file = line_vector(&[
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
		assert_eq!(btree_set_of_tags(&["$TAG", "$TOG"]), post.tags);
		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
		let content = (post.content)().collect::<Vec<_>>();
		let expected_content = line_vector(&[
			"",
			"Lorem ipsum dolor sit amet.",
			"Ut enim ad minim veniam.",
			"Duis aute irure dolor in reprehenderit.",
			"Excepteur sint occaecat cupidatat non proident.",
		]);
		assert_eq!(expected_content, content);
	}

	fn line_vector(lines: &[&str]) -> Vec<String> {
		lines.iter().copied().map(str::to_string).collect()
	}
}
