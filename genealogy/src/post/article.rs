use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::content::Content;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::raw_post::{RawPost, DATE, DESCRIPTION, REPOSITORY, SLUG, TAGS, TITLE};
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use chrono::NaiveDate;
use debug_stub_derive::DebugStub;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(DebugStub)]
pub struct Article {
	pub title: Title,
	pub tags: HashSet<Tag>,
	pub date: NaiveDate,
	pub description: Description,
	pub slug: Slug,
	pub repository: Option<Repository>,
	#[debug_stub = "Content"]
	pub content: Content,
}

// RUSTIFICATION: Implement PartialEq etc. for ALL Posts in one single impl block.
impl PartialEq for Article {
	fn eq(&self, other: &Self) -> bool {
		self.slug.eq(&other.slug)
	}
}

impl Eq for Article {}

// NOTE: Not part of the original, but very helpful.
impl PartialOrd for Article {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

// NOTE: Not part of the original, but very helpful.
impl Ord for Article {
	fn cmp(&self, other: &Self) -> Ordering {
		self.slug.cmp(&other.slug)
	}
}

impl Hash for Article {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.slug.hash(state)
	}
}

impl TryFrom<&Path> for Article {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		RawPost::try_from(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating article failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Article::try_from)
	}
}

impl TryFrom<Vec<String>> for Article {
	type Error = Exception;

	fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
		RawPost::try_from(lines).and_then(Article::try_from)
	}
}

impl TryFrom<RawPost> for Article {
	type Error = Exception;

	fn try_from(raw_post: RawPost) -> Result<Self, Self::Error> {
		let front_matter = raw_post.front_matter;
		// RUSTIFICATION: Create a trait that allows simple text parsing and
		// put the constants in it as associated const so they can be used by
		// dynamic code for lookup in the front matter.
		Ok(Article {
			title: Title::from_text(front_matter.value_of(TITLE)?)?,
			tags: Tag::from(front_matter.value_of(TAGS)?)?,
			date: parse_date(front_matter.value_of(DATE)?)?,
			description: Description::from_text(front_matter.value_of(DESCRIPTION)?)?,
			slug: Slug::new(front_matter.value_of(SLUG)?.to_string())?,
			repository: front_matter
				.value_of(REPOSITORY)
				.ok()
				.map(str::to_string)
				.map(Repository::from_identifier)
				.transpose()?,
			content: raw_post.content,
		})
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helpers::hash_set_of_tags;
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

		let post = Article::try_from(file).unwrap();

		assert_eq!("Cool: A blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags);
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

		let post = Article::try_from(file).unwrap();

		assert_eq!("A cool blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags);
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

		let post = Article::try_from(file).unwrap();

		assert_eq!("A cool blog post", post.title.text);
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags);
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
