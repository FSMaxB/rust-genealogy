use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Tag {
	pub text: String,
}

impl Tag {
	pub fn from_text(tags_text: &str) -> HashSet<Tag> {
		lazy_static! {
			static ref SQUARE_BRACKET_REGEX: Regex = Regex::new("^\\[|\\]$").unwrap();
		}

		SQUARE_BRACKET_REGEX
			.replace_all(tags_text, "")
			.as_ref()
			.split(',')
			.map(str::trim)
			.filter(|tag_text| !tag_text.is_empty())
			.map(str::to_owned)
			.map(|text| Tag { text })
			.collect()
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

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}

	#[test]
	fn single_element_array_single_tag() {
		let tags_text = "[$TAG]";
		let expected_tags = hash_set_of_tags(&["$TAG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}

	#[test]
	fn multiple_elements_array_multiple_tags() {
		let tags_text = "[$TAG,$TOG,$TUG]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TOG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}

	#[test]
	fn multiple_elements_array_with_spaces_multiple_tags_without_spaces() {
		let tags_text = "[$TAG ,  $TOG , $TUG  ]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TOG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}

	#[test]
	fn multiple_elements_array_with_just_spaces_tag_empty_tag_is_ignored() {
		let tags_text = "[$TAG ,  , $TUG  ]";
		let expected_tags = hash_set_of_tags(&["$TAG", "$TUG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}

	#[test]
	fn multiple_elements_array_duplicate_tags_duplicate_tag_is_ignored() {
		let tags_text = "[$TAG, $TAG]";
		let expected_tags = hash_set_of_tags(&["$TAG"]);

		assert_eq!(expected_tags, Tag::from_text(tags_text));
	}
}
