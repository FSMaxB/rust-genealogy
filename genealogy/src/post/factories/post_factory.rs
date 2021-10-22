use crate::helpers::collector::Collectors;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgumentException, RuntimeException};
use crate::helpers::extensions::Extensions;
use crate::helpers::indexing::index;
use crate::helpers::stream::{Stream, StreamExtensions};
use crate::helpers::string_extensions::StringExtensions;
use crate::post::factories::raw_front_matter::RawFrontMatter;
use crate::post::factories::raw_post::RawPost;
use crate::throw;
use crate::utils::Utils;
use std::path::Path;

/// ```java
/// final class PostFactory {
/// 	private PostFactory() {
///			// private constructor to prevent accidental instantiation of utility class
///		}
/// ```
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub(super) enum PostFactory {}

impl PostFactory {
	/// ```java
	/// public static final String DATE = "date";
	/// ```
	pub const DATE: &'static str = "date";

	/// ```java
	///	public static final String DESCRIPTION = "description";
	/// ```
	pub const DESCRIPTION: &'static str = "description";

	/// ```java
	///	public static final String REPOSITORY = "repo";
	/// ```
	pub const REPOSITORY: &'static str = "repo";

	/// ```java
	///	public static final String SLIDES = "slides";
	/// ```
	pub const SLIDES: &'static str = "slides";

	/// ```java
	///	public static final String SLUG = "slug";
	/// ```
	pub const SLUG: &'static str = "slug";

	/// ```java
	///	public static final String TAGS = "tags";
	/// ```
	pub const TAGS: &'static str = "tags";

	/// ```java
	///	public static final String TITLE = "title";
	/// ```
	pub const TITLE: &'static str = "title";

	/// ```java
	///	public static final String VIDEO = "videoSlug";
	/// ```
	pub const VIDEO: &'static str = "videoSlug";

	/// ```java
	///	private static final String FRONT_MATTER_SEPARATOR = "---";
	/// ```
	const FRONT_MATTER_SEPARATOR: &'static str = "---";

	/// ```java
	/// public static RawPost readPost(Path file) {
	///		try {
	///			List<String> eagerLines = Utils.uncheckedFilesReadAllLines(file);
	///			return readPost(eagerLines);
	///		} catch (RuntimeException ex) {
	///			throw new RuntimeException("Creating article failed: " + file, ex);
	///		}
	///	}
	/// ````
	/// Note: Different name in rust because overloading isn't possible
	pub fn read_post_from_path(file: &Path) -> Result<RawPost, Exception> {
		// simulated try-catch block
		(|| {
			let eager_lines = Utils::unchecked_files_read_all_lines(file)?;
			Self::read_post(eager_lines)
		})()
		.map_err(|exception| RuntimeException(format!("Creating article failed: {:?}", file), exception.into()))
	}

	/// ```java
	/// public static RawPost readPost(List<String> fileLines) {
	///		RawFrontMatter frontMatter = extractFrontMatter(fileLines);
	///		Content content = () -> extractContent(fileLines);
	///		return new RawPost(frontMatter, content);
	///	}
	/// ```
	pub fn read_post(file_lines: Vec<String>) -> Result<RawPost, Exception> {
		let front_matter = Self::extract_front_matter(file_lines.clone())?;
		let content = Box::new(move || Self::extract_content(file_lines));
		Ok(RawPost::new(front_matter, content))
	}

	/// ```java
	/// private static RawFrontMatter extractFrontMatter(List<String> fileLines) {
	///		Map<String, String> frontMatter = readFrontMatter(fileLines)
	///				.filter(line -> !line.startsWith("#"))
	///				.map(PostFactory::keyValuePairFrom)
	///				.collect(toMap(FrontMatterLine::key, FrontMatterLine::value));
	///		return new RawFrontMatter(frontMatter);
	///	}
	/// ```
	fn extract_front_matter(file_lines: Vec<String>) -> Result<RawFrontMatter, Exception> {
		let front_matter = Self::read_front_matter(file_lines)
			.filter(|line| !line.starts_with("#"))
			.map(|string| PostFactory::key_value_pair_from(&string))
			.collect(Collectors::to_map(FrontMatterLine::key, FrontMatterLine::value))?;
		Ok(RawFrontMatter::new(front_matter))
	}

	/// ```java
	///	private static Stream<String> readFrontMatter(List<String> markdownFile) {
	///		return markdownFile.stream()
	///				.map(String::strip)
	///				.dropWhile(not(FRONT_MATTER_SEPARATOR::equals))
	///				.skip(1)
	///				.takeWhile(not(FRONT_MATTER_SEPARATOR::equals));
	///	}
	/// ```
	fn read_front_matter(markdown_file: Vec<String>) -> Stream<String> {
		markdown_file
			.stream()
			.map(|string| Ok(string.strip()))
			.drop_while(|string| string != Self::FRONT_MATTER_SEPARATOR)
			.skip(1)
			.take_while(|string| string != Self::FRONT_MATTER_SEPARATOR)
	}

	/// ```java
	///	private static FrontMatterLine keyValuePairFrom(String line) {
	///		String[] pair = line.split(":", 2);
	///		if (pair.length < 2)
	///			throw new IllegalArgumentException("Line doesn't seem to be a key/value pair (no colon): " + line);
	///		String key = pair[0].strip();
	///		if (key.isBlank())
	///			throw new IllegalArgumentException("Line \"" + line + "\" has no key.");
	///
	///		String value = pair[1].strip();
	///		return new FrontMatterLine(key, value);
	///	}
	/// ```
	fn key_value_pair_from(line: &str) -> Result<FrontMatterLine, Exception> {
		let pair = line.split_limit(':', 2);
		if pair.len() < 2 {
			throw!(IllegalArgumentException(format!(
				"Line doesn't seem to be a key/value pair (no colon): {}",
				line
			)));
		}
		let key = index(&pair, 0)?.strip();
		if key.is_blank() {
			throw!(IllegalArgumentException(format!(r#"Line "{}" has no key."#, line)));
		}

		let value = index(&pair, 1)?.strip();
		Ok(FrontMatterLine::new(key, value))
	}

	/// ```java
	/// private static Stream<String> extractContent(List<String> markdownFile) {
	///		return markdownFile.stream()
	///				.dropWhile(line -> !line.strip().equals(FRONT_MATTER_SEPARATOR))
	///				.skip(1)
	///				.dropWhile(line -> !line.strip().equals(FRONT_MATTER_SEPARATOR))
	///				.skip(1);
	///	}
	/// ```
	fn extract_content(markdown_file: Vec<String>) -> Stream<String> {
		markdown_file
			.stream()
			.drop_while(|line| !line.strip().equals(Self::FRONT_MATTER_SEPARATOR))
			.skip(1)
			.drop_while(|line| !line.strip().equals(Self::FRONT_MATTER_SEPARATOR))
			.skip(1)
	}
}

/// ```java
/// private record FrontMatterLine(String key, String value) { }
/// ```
struct FrontMatterLine {
	key: String,
	value: String,
}

impl FrontMatterLine {
	/// ```java
	/// private record FrontMatterLine(String key, String value) { }
	/// ```
	fn new(key: String, value: String) -> Self {
		Self { key, value }
	}

	/// NOTE: For use as to_map key mapper.
	fn key(&self) -> String {
		self.key.clone()
	}

	/// NOTE: For use as to_map value mapper.
	fn value(&self) -> String {
		self.value.clone()
	}
}
