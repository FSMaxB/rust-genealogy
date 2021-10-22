use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::RuntimeException;
use crate::post::article::Article;
use crate::post::description::Description;
use crate::post::factories::parse_date;
use crate::post::factories::post_factory::{DATE, DESCRIPTION, REPOSITORY, SLUG, TAGS, TITLE};
use crate::post::factories::raw_post::RawPost;
use crate::post::repository::Repository;
use crate::post::slug::Slug;
use crate::post::tag::Tag;
use crate::post::title::Title;
use std::path::Path;

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
		let front_matter = raw_post.front_matter();
		// RUSTIFICATION: Create a trait that allows simple text parsing and
		// put the constants in it as associated const so they can be used by
		// dynamic code for lookup in the front matter.
		Ok(Article::new(
			Title::new(&front_matter.required_value_of(TITLE)?)?,
			Tag::from(&front_matter.required_value_of(TAGS)?)?,
			parse_date(&front_matter.required_value_of(DATE)?)?,
			Description::new(&front_matter.required_value_of(DESCRIPTION)?)?,
			Slug::new(front_matter.required_value_of(SLUG)?)?,
			front_matter
				.required_value_of(REPOSITORY)
				.ok()
				.map(Repository::new)
				.transpose()?,
			raw_post.content(),
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
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
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
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
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
		assert_eq!(hash_set_of_tags(&["$TAG", "$TOG"]), post.tags());
		assert_eq!(LocalDate::from_ymd(2020, 1, 23), post.date);
		assert_eq!("Very blog, much post, so wow", post.description.text);
		assert_eq!("cool-blog-post", post.slug.value);
		let content = (post.content)().to_list().unwrap();
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
