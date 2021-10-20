use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgumentException, RuntimeException};
use crate::post::content::Content;
use crate::post::factories::raw_front_matter::RawFrontMatter;
use crate::utils::unchecked_files_read_all_lines;
use std::convert::TryFrom;
use std::path::Path;

pub struct RawPost {
	// NOTE: Using `pub` here instead of getters, because Java doesn't provide
	// any more privacy anyways since you can still modify them via the reference.
	// This also means we don't need a constructor!
	pub front_matter: RawFrontMatter,
	pub content: Content,
}

pub const DATE: &str = "date";
pub const DESCRIPTION: &str = "description";
pub const REPOSITORY: &str = "repo";
pub const SLIDES: &str = "slides";
pub const SLUG: &str = "slug";
pub const TAGS: &str = "tags";
pub const TITLE: &str = "title";
pub const VIDEO: &str = "videoSlug";

impl TryFrom<&Path> for RawPost {
	type Error = Exception;

	fn try_from(path: &Path) -> Result<Self, Self::Error> {
		unchecked_files_read_all_lines(path)
			.map_err(|error| {
				RuntimeException(format!(
					r#"Creating article failed: "{}", error: {}"#,
					path.to_string_lossy(),
					error
				))
			})
			.and_then(Self::try_from)
	}
}

impl TryFrom<Vec<String>> for RawPost {
	type Error = Exception;

	fn try_from(lines: Vec<String>) -> Result<Self, Self::Error> {
		let front_matter = extract_front_matter(lines.clone())?; // FIXME: Cloning for now, but do it properly with slices and lifetimes later.
		let content = Box::new(move || extract_content(lines));
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
	let colon_index = line.find(':').ok_or_else(|| {
		IllegalArgumentException(format!("Line doesn't seem to be a key/value pair (no colon): {}", line))
	})?;
	let (key, value) = line.split_at(colon_index);
	let key = key.trim().to_string();
	// The value still has the leading colon in it, so it needs to be removed.
	let value = (&value[1..].trim()).to_string();

	if key.is_empty() {
		return Err(IllegalArgumentException(format!(r#"Line "{}" has no key"#, line)));
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
