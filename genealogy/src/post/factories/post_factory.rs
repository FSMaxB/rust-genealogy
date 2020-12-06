use crate::java_replicas::exception::Exception;
use crate::java_replicas::exception::Exception::{IllegalArgument, RuntimeException};
use crate::post::factories::raw_front_matter::RawFrontMatter;
use crate::post::factories::raw_post::RawPost;
use crate::utils::unchecked_files_read_all_lines;
use std::path::PathBuf;

pub struct PostFactory;

impl PostFactory {
	#[allow(dead_code)]
	pub const DATE: &'static str = "date";
	#[allow(dead_code)]
	pub const DESCRIPTION: &'static str = "description";
	#[allow(dead_code)]
	pub const REPOSITORY: &'static str = "repo";
	#[allow(dead_code)]
	pub const SLIDES: &'static str = "slides";
	#[allow(dead_code)]
	pub const SLUG: &'static str = "slug";
	#[allow(dead_code)]
	pub const TAGS: &'static str = "tags";
	#[allow(dead_code)]
	pub const TITLE: &'static str = "title";
	#[allow(dead_code)]
	pub const VIDEO: &'static str = "videoSlug";

	#[allow(dead_code)]
	pub fn read_post_from_path(file: &PathBuf) -> Result<RawPost, Exception> {
		unchecked_files_read_all_lines(file)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating article failed: "{}", error: {}"#,
					file.to_string_lossy(),
					error
				))
			})
			.and_then(Self::read_post_from_lines)
	}

	#[allow(dead_code)]
	pub fn read_post_from_lines(file_lines: Vec<String>) -> Result<RawPost, Exception> {
		let front_matter = extract_front_matter(file_lines.clone())?; // FIXME: Cloning for now, but do it properly with slices and lifetimes later.
		let content = Box::new(move || extract_content(file_lines));
		Ok(RawPost { front_matter, content })
	}
}

fn extract_front_matter(file_lines: Vec<String>) -> Result<RawFrontMatter, Exception> {
	read_front_matter(file_lines)
		.map(key_value_pair_from)
		// NOTE: Collecting into `Result` allows collecting the map and isolating the exception case at the same time.
		// This works because `Result` implements the `FromIterator` trait, short circuiting on the first error it encounters.
		.collect::<Result<_, _>>()
		.map(RawFrontMatter::new)
}

fn read_front_matter(markdown_file: Vec<String>) -> impl Iterator<Item = String> {
	markdown_file
		.into_iter()
		.map(|line| line.trim().to_string())
		.skip_while(|line| line != FRONT_MATTER_SEPARATOR)
		.skip(1)
		.take_while(|line| line != FRONT_MATTER_SEPARATOR)
}

// NOTE: FrontMatterLine is not necessary because `Iterator::collect` works on Tuples.
fn key_value_pair_from(line: String) -> Result<(String, String), Exception> {
	let colon_index = line
		.find(':')
		.ok_or_else(|| IllegalArgument(format!("Line doesn't seem to be a key/value pair (no colon): {}", line)))?;
	let (key, value) = line.split_at(colon_index);
	let key = key.trim().to_string();
	// The value still has the leading colon in it, so it needs to be removed.
	let value = (&value[1..].trim()).to_string();

	if key.is_empty() {
		return Err(IllegalArgument(format!(r#"Line "{}" has no key"#, line)));
	}

	Ok((key, value))
}

fn extract_content(markdown_file: Vec<String>) -> Box<dyn Iterator<Item = String>> {
	Box::new(
		markdown_file
			.into_iter()
			.map(|line| line.trim().to_string())
			.skip_while(|line| line != FRONT_MATTER_SEPARATOR)
			.skip(1)
			.skip_while(|line| line != FRONT_MATTER_SEPARATOR)
			.skip(1),
	)
}

const FRONT_MATTER_SEPARATOR: &str = "---";
