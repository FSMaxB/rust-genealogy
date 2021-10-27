use genealogy_java_apis::collector::Collectors;
use genealogy_java_apis::exception::Exception;
use genealogy_java_apis::record;
use genealogy_java_apis::set::Set;
use genealogy_java_apis::stream::Stream;
use genealogy_java_apis::string::JString;

/// ```java
/// public record Tag(String text) {
/// 	public Tag {
/// 		requireNonNull(text);
/// 	}
/// ```
#[record]
pub struct Tag {
	text: JString,
}

impl Tag {
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
	pub fn from(tags_text: JString) -> Result<Set<Tag>, Exception> {
		Stream::of(tags_text.replace_all("^\\[|\\]$", "")?.split(','))
			.map(|string| Ok(string.strip()))
			.filter(|string| !string.is_empty())
			.map(|string| Ok(Tag::new(string)))
			// An UnmodifiableSet isn't really necessary in rust since it
			// can only be modified via mutable reference anyways.
			.collect(Collectors::to_unmodifiable_set())
	}
}

/// ```java
/// class TagTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use genealogy_java_apis::string::jstrings;
	use genealogy_java_apis::test::assert_that;

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
	pub(super) fn empty_element_array__empty_tag() {
		let tags_text = "[ ]".into();
		let expected_tags = jstrings([]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn single_element_array__single_tag() {
		let tags_text = "[$TAG]".into();
		let expected_tags = jstrings(["$TAG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn multiple_elements_array__multiple_tags() {
		let tags_text = "[$TAG,$TOG,$TUG]".into();
		let expected_tags = jstrings(["$TAG", "$TOG", "$TUG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn multiple_elements_array_with_spaces_multiple__tags_without_spaces() {
		let tags_text = "[$TAG ,  $TOG , $TUG  ]".into();
		let expected_tags = jstrings(["$TAG", "$TOG", "$TUG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn multiple_elements_array_with_just_spaces_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,  , $TUG  ]".into();
		let expected_tags = jstrings(["$TAG", "$TUG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn multiple_elements_array_with_empty_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,, $TUG ]".into();
		let expected_tags = jstrings(["$TAG", "$TUG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
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
	pub(super) fn multiple_elements_array_duplicate_tags_duplicate_tag_is_ignored() {
		let tags_text = "[$TAG, $TAG]".into();
		let expected_tags = jstrings(["$TAG"]);

		let tags = Tag::from(tags_text).unwrap();

		assert_that(tags)
			.extracting(Tag::text)
			.contains_exactly_in_any_order(expected_tags);
	}
}
