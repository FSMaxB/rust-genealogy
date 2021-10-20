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
	pub fn from_text(tags_text: &str) -> Result<HashSet<Tag>, Exception> {
		Stream::of(tags_text.replace_all("^\\[|\\]$", "")?.split(','))
			.map(|string| string.strip())
			.filter(|string| !string.is_empty())
			.map(Tag::new)
			// An UnmodifiableSet isn't really necessary in rust since it
			// can only be modified via mutable reference anyways.
			.collect(Collectors::to_hash_set())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_helpers::hash_set_of_tags;

	#[test]
	fn empty_element_array_empty_tag() {
		let tags_text = "[ ]";
		let expected_tags = HashSet::default();

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}

	#[test]
	fn single_element_array_single_tag() {
		let tags_text = "[$TAG]";
		let expected_tags = hash_set_of_tags(&["$TAG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}

	#[test]
	fn multiple_elements_array_multiple_tags() {
		let tags_text = "[$TAG,$TOG,$TUG]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TOG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}

	#[test]
	fn multiple_elements_array_with_spaces_multiple_tags_without_spaces() {
		let tags_text = "[$TAG ,  $TOG , $TUG  ]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TOG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}

	#[test]
	fn multiple_elements_array_with_just_spaces_tag_empty_tag_is_ignored() {
		let tags_text = "[$TAG ,  , $TUG  ]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}

	#[test]
	fn multiple_elements_array_duplicate_tags_duplicate_tag_is_ignored() {
		let tags_text = "[$TAG, $TAG]";
		let expected_tags = hash_set_of_tags(&["$TAG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text).unwrap());
	}
}
