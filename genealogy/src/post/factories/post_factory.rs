use crate::helpers::collector::Collectors;
use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::{IllegalArgumentException, RuntimeException};
use crate::helpers::list::List;
use crate::helpers::path::Path;
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use crate::post::factories::raw_front_matter::RawFrontMatter;
use crate::post::factories::raw_post::RawPost;
use crate::r#static;
use crate::throw;
use crate::utils::Utils;
/// ```java
/// final class PostFactory {
/// 	private PostFactory() {
///			// private constructor to prevent accidental instantiation of utility class
///		}
/// ```
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub(super) enum PostFactory {}

impl PostFactory {
	// ```java
	// public static final String DATE = "date";
	// ```
	r#static!(pub DATE: JString = "date".into());

	// ```java
	//	public static final String DESCRIPTION = "description";
	// ```
	r#static!(pub DESCRIPTION: JString = "description".into());

	// ```java
	//	public static final String REPOSITORY = "repo";
	// ```
	r#static!(pub REPOSITORY: JString = "repo".into());

	// ```java
	//	public static final String SLIDES = "slides";
	// ```
	r#static!(pub SLIDES: JString = "slides".into());

	// ```java
	//	public static final String SLUG = "slug";
	// ```
	r#static!(pub SLUG: JString = "slug".into());

	// ```java
	//	public static final String TAGS = "tags";
	// ```
	r#static!(pub TAGS: JString = "tags".into());

	// ```java
	//	public static final String TITLE = "title";
	// ```
	r#static!(pub TITLE: JString = "title".into());

	// ```java
	//	public static final String VIDEO = "videoSlug";
	// ```
	r#static!(pub VIDEO: JString = "videoSlug".into());

	// ```java
	//	private static final String FRONT_MATTER_SEPARATOR = "---";
	// ```
	r#static!(FRONT_MATTER_SEPARATOR: JString = "---".into());

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
	pub fn read_post_from_path(file: Path) -> Result<RawPost, Exception> {
		// simulated try-catch block
		(|| {
			let eager_lines = Utils::unchecked_files_read_all_lines(file.clone())?;
			Self::read_post(eager_lines)
		})()
		.map_err(|exception| RuntimeException("Creating article failed: " + file, exception.into()))
	}

	/// ```java
	/// public static RawPost readPost(List<String> fileLines) {
	///		RawFrontMatter frontMatter = extractFrontMatter(fileLines);
	///		Content content = () -> extractContent(fileLines);
	///		return new RawPost(frontMatter, content);
	///	}
	/// ```
	pub fn read_post(file_lines: List<JString>) -> Result<RawPost, Exception> {
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
	fn extract_front_matter(file_lines: List<JString>) -> Result<RawFrontMatter, Exception> {
		let front_matter = Self::read_front_matter(file_lines)
			.filter(|line| !line.starts_with("#"))
			.map(PostFactory::key_value_pair_from)
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
	fn read_front_matter(markdown_file: List<JString>) -> Stream<'static, JString> {
		markdown_file
			.stream()
			.map(|string| Ok(string.strip()))
			.drop_while(|string| string != Self::FRONT_MATTER_SEPARATOR())
			.skip(1)
			.take_while(|string| string != Self::FRONT_MATTER_SEPARATOR())
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
	fn key_value_pair_from(line: JString) -> Result<FrontMatterLine, Exception> {
		let pair = line.split_limit(':', 2);
		if pair.len() < 2 {
			throw!(IllegalArgumentException(
				"Line doesn't seem to be a key/value pair (no colon): " + line
			));
		}
		let key = pair.get(0)?.strip();
		if key.is_blank() {
			throw!(IllegalArgumentException(r#"Line ""# + line + r#"" has no key."#));
		}

		let value = pair.get(1)?.strip();
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
	fn extract_content(markdown_file: List<JString>) -> Stream<'static, JString> {
		markdown_file
			.stream()
			.drop_while(|line| line.strip() != Self::FRONT_MATTER_SEPARATOR())
			.skip(1)
			.drop_while(|line| line.strip() != Self::FRONT_MATTER_SEPARATOR())
			.skip(1)
	}
}

/// ```java
/// private record FrontMatterLine(String key, String value) { }
/// ```
struct FrontMatterLine {
	key: JString,
	value: JString,
}

impl FrontMatterLine {
	/// ```java
	/// private record FrontMatterLine(String key, String value) { }
	/// ```
	fn new(key: JString, value: JString) -> Self {
		Self { key, value }
	}

	/// NOTE: For use as to_map key mapper.
	fn key(&self) -> JString {
		self.key.clone()
	}

	/// NOTE: For use as to_map value mapper.
	fn value(&self) -> JString {
		self.value.clone()
	}
}
