use crate::helpers::collector::Collectors;
use crate::helpers::exception::Exception;
use crate::helpers::stream::Stream;
use crate::helpers::string_extensions::StringExtensions;
use std::collections::HashSet;

/// ```java
/// public record Tag(String text) {
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tag {
	pub text: String,
}

impl Tag {
	/// ```java
	/// public Tag {
	/// 	requireNonNull(text);
	/// }
	/// ```
	pub fn new(text: String) -> Tag {
		Tag { text }
	}

	/// ```java
	/// public static Set<Tag> from(String tagsText) {
	///		return Stream.of(tagsText
	///				.replaceAll("^\\[|\\]$", "")
	///				.split(","))
	///				.map(String::strip)
	///				.filter(not(String::isBlank))
	///				.map(Tag::new)
	///				.collect(toUnmodifiableSet());
	///	}
	/// ```
	pub fn from(tags_text: &str) -> Result<HashSet<Tag>, Exception> {
		Stream::of(tags_text.replace_all("^\\[|\\]$", "")?.split(','))
			.map(|string| string.strip())
			.filter(|string| !string.is_empty())
			.map(Tag::new)
			// An UnmodifiableSet isn't really necessary in rust since it
			// can only be modified via mutable reference anyways.
			.collect(Collectors::to_hash_set())
	}
}

/// ```java
/// class TagTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;

	/// ```java
	/// @Test
	///	void emptyElementArray_emptyTag() {
	///		var tagsText = "[ ]";
	///		var expectedTags = new String[] { };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn empty_element_array__empty_tag() {
		let tags_text = "[ ]";
		let expected_tags: [&str; 0] = [];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void singleElementArray_singleTag() {
	///		var tagsText = "[$TAG]";
	///		var expectedTags = new String[] { "$TAG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn single_element_array__single_tag() {
		let tags_text = "[$TAG]";
		let expected_tags = ["$TAG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void multipleElementsArray_multipleTags() {
	///		var tagsText = "[$TAG,$TOG,$TUG]";
	///		var expectedTags = new String[]{ "$TAG", "$TOG", "$TUG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn multiple_elements_array__multiple_tags() {
		let tags_text = "[$TAG,$TOG,$TUG]";
		let expected_tags = ["$TAG", "$TOG", "$TUG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void multipleElementsArrayWithSpaces_multipleTagsWithoutSpaces() {
	///		var tagsText = "[$TAG ,  $TOG , $TUG  ]";
	///		var expectedTags = new String[]{ "$TAG", "$TOG", "$TUG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn multiple_elements_array_with_spaces_multiple__tags_without_spaces() {
		let tags_text = "[$TAG ,  $TOG , $TUG  ]";
		let expected_tags = ["$TAG", "$TOG", "$TUG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void multipleElementsArrayWithJustSpacesTag_emptyTagIsIgnored() {
	///		var tagsText = "[$TAG ,  , $TUG  ]";
	///		var expectedTags = new String[]{ "$TAG", "$TUG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn multiple_elements_array_with_just_spaces_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,  , $TUG  ]";
		let expected_tags = ["$TAG", "$TUG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void multipleElementsArrayWithEmptyTag_emptyTagIsIgnored() {
	///		var tagsText = "[$TAG ,, $TUG  ]";
	///		var expectedTags = new String[]{ "$TAG", "$TUG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn multiple_elements_array_with_empty_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,, $TUG ]";
		let expected_tags = ["$TAG", "$TUG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}

	/// ```java
	/// @Test
	///	void multipleElementsArrayDuplicateTags_duplicateTagIsIgnored() {
	///		var tagsText = "[$TAG, $TAG]";
	///		var expectedTags = new String[]{ "$TAG" };
	///
	///		var tags = Tag.from(tagsText);
	///
	///		Assertions.assertThat(tags)
	///				.extracting(Tag::text)
	///				.containsExactlyInAnyOrder(expectedTags);
	///	}
	/// ```
	#[test]
	fn multiple_elements_array_duplicate_tags_duplicate_tag_is_ignored() {
		let tags_text = "[$TAG, $TAG]";
		let expected_tags = ["$TAG"];

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(|tag| tag.text)
			.contains_exactly_in_any_order(expected_tags);
	}
}
