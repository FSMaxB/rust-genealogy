use crate::article::content::Content;
use crate::article::description::Description;
use crate::article::slug::Slug;
use crate::article::tag::Tag;
use crate::article::title::Title;
use crate::article::Article;
use crate::exception::Exception;
use crate::try_iterator::TryIterator;
use crate::utils::file_lines;
use chrono::NaiveDate;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::Error;
use std::iter::Iterator;
use std::path::Path;

pub enum ArticleFactory {}

impl ArticleFactory {
	// FIXME: Is it a bug that two of these are public in the original implementation?
	const TITLE: &'static str = "title";
	const TAGS: &'static str = "tags";
	pub const DATE: &'static str = "date";
	const DESCRIPTION: &'static str = "description";
	const SLUG: &'static str = "slug";
	pub const FRONT_MATTER_SEPARATOR: &'static str = "---";

	pub fn create_from_file(file: &Path) -> Result<Article, Exception> {
		let eager_lines = file_lines(file);
		let front_matter = Self::extract_front_matter(eager_lines);
		let owned_path = file.to_path_buf();
		let content = Content::from(move || {
			let lazy_lines = file_lines(&owned_path);
			Self::extract_content(lazy_lines)
		});

		Self::create_from_iterators(front_matter, content).map_err(|exception| {
			Exception::Runtime(format!(
				"Creating article failed: {}, exception: {}",
				file.to_string_lossy(),
				exception
			))
		})
	}

	pub fn create_from_lines(lines: Vec<String>) -> Article {
		let front_matter = Self::extract_front_matter(lines.clone().into_iter().map(Ok));
		let content = Content::from(move || Self::extract_content(lines.clone().into_iter().map(Ok)));
		//This unwrap can't fail because we created iterators that always return Ok(String)
		Self::create_from_iterators(front_matter, content).unwrap()
	}

	pub fn create_from_iterators(
		front_matter: impl Iterator<Item = Result<String, std::io::Error>>,
		content: Content,
	) -> Result<Article, Exception> {
		let mut entries: BTreeMap<String, String> = front_matter
			.map(FrontMatterLine::try_from)
			.map_ok(|line| (line.key, line.value))
			.collect::<Result<_, Exception>>()?;

		let title_entry = entries.remove(Self::TITLE).ok_or(Exception::NullPointer)?;
		let tags_entry = entries.remove(Self::TAGS).ok_or(Exception::NullPointer)?;
		let date_entry = entries.remove(Self::DATE).ok_or(Exception::NullPointer)?;
		let description_entry = entries.remove(Self::DESCRIPTION).ok_or(Exception::NullPointer)?;
		let slug_entry = entries.remove(Self::SLUG).ok_or(Exception::NullPointer)?;

		Ok(Article {
			title: Title::try_from(title_entry)?,
			tags: Tag::set_from_text(&tags_entry)?,
			date: NaiveDate::parse_from_str(&date_entry, "%Y-%m-%d").map_err(|_| Exception::DateTimeParse)?,
			description: Description::try_from(description_entry)?,
			slug: Slug::try_from(slug_entry)?,
			content,
		})
	}

	fn extract_front_matter(
		markdown_file: impl Iterator<Item = Result<String, std::io::Error>>,
	) -> impl Iterator<Item = Result<String, std::io::Error>> {
		markdown_file
			.map_ok(|string| string.trim().to_string())
			.try_skip_while(|line| line != Self::FRONT_MATTER_SEPARATOR)
			.try_skip(1)
			.try_take_while(|line| line != Self::FRONT_MATTER_SEPARATOR)
	}

	fn extract_content(
		markdown_file: impl Iterator<Item = Result<String, std::io::Error>>,
	) -> impl Iterator<Item = Result<String, std::io::Error>> {
		markdown_file
			.try_skip_while(|line| line.trim() != Self::FRONT_MATTER_SEPARATOR)
			.try_skip(1)
			.try_skip_while(|line| line.trim() != Self::FRONT_MATTER_SEPARATOR)
			.try_skip(1)
	}
}

struct FrontMatterLine {
	key: String,
	value: String,
}

impl TryFrom<Result<String, std::io::Error>> for FrontMatterLine {
	type Error = Exception;

	fn try_from(line: Result<String, Error>) -> Result<Self, Self::Error> {
		let line = line?; // propagate Exception
		let pair: Vec<_> = line.splitn(2, ":").collect();
		if pair.len() < 2 {
			return Err(Exception::IllegalArgument(format!(
				"Line doesn't seem to be a key/value pair (no colon): {}",
				line
			)));
		}

		let key = pair[0].trim().to_lowercase();
		if key.is_empty() {
			return Err(Exception::IllegalArgument(format!(r#"Line "{}" has no key."#, line)));
		}
		let value = pair[1].trim().to_string();
		Ok(FrontMatterLine { key, value })
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::collections::HashSet;

	// This comment is taken verbatim from the java example
	/*
	 * TODO: tests for...
	 *  - lines without colon
	 *  - lines with empty key
	 *  - lines with empty value
	 *  - missing lines
	 *  - superfluous lines
	 */

	#[test]
	#[allow(non_snake_case)]
	fn create_from_front_matter__multiple_colons__get_valid_article() {
		let front_matter = vec![
			"title: Cool: A blog post",
			"tags: [$TAG, $TOG]",
			"date: 2020-01-23",
			"description: \"Very blog, much post, so wow\"",
			"slug: cool-blog-post",
		]
		.into_iter()
		.map(ToString::to_string)
		.map(Ok);

		let article = ArticleFactory::create_from_iterators(front_matter, Content::from(std::iter::empty)).unwrap();
		assert_eq!("Cool: A blog post", article.title.text);

		let expected_tags: HashSet<_> = vec!["$TAG", "$TOG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();
		assert_eq!(expected_tags, article.tags);

		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), article.date);
		assert_eq!("Very blog, much post, so wow", article.description.text);
		assert_eq!("cool-blog-post", article.slug.value);
	}

	#[test]
	#[allow(non_snake_case)]
	fn create_from_front_matter__all_tags_correct__get_valid_article() {
		let front_matter = vec![
			"title: A blog post",
			"tags: [$TAG, $TOG]",
			"date: 2020-01-23",
			"description: \"Very blog, much post, so wow\"",
			"slug: cool-blog-post",
		]
		.into_iter()
		.map(ToString::to_string)
		.map(Ok);

		let article = ArticleFactory::create_from_iterators(front_matter, Content::from(std::iter::empty)).unwrap();
		assert_eq!("A blog post", article.title.text);

		let expected_tags: HashSet<_> = vec!["$TAG", "$TOG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();
		assert_eq!(expected_tags, article.tags);

		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), article.date);
		assert_eq!("Very blog, much post, so wow", article.description.text);
		assert_eq!("cool-blog-post", article.slug.value);
	}

	#[test]
	#[allow(non_snake_case)]
	fn create_from_file__all_tags_correct__get_valid_article() {
		let file = vec![
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
		]
		.into_iter()
		.map(ToString::to_string)
		.collect();

		let article = ArticleFactory::create_from_lines(file);
		assert_eq!("A cool blog post", article.title.text);

		let expected_tags: HashSet<_> = vec!["$TAG", "$TOG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();
		assert_eq!(expected_tags, article.tags);

		assert_eq!(NaiveDate::from_ymd(2020, 1, 23), article.date);
		assert_eq!("Very blog, much post, so wow", article.description.text);
		assert_eq!("cool-blog-post", article.slug.value);

		let expected_content: Vec<_> = vec![
			"",
			"Lorem ipsum dolor sit amet.",
			"Ut enim ad minim veniam.",
			"Duis aute irure dolor in reprehenderit.",
			"Excepteur sint occaecat cupidatat non proident.",
		]
		.into_iter()
		.map(ToString::to_string)
		.collect();
		let actual_content: Vec<_> = (article.content)().collect::<Result<_, _>>().unwrap();
		assert_eq!(expected_content, actual_content);
	}
}
