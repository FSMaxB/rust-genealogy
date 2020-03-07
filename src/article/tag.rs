use crate::exception::Exception;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeSet;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tag {
	pub text: String,
}

impl TryFrom<String> for Tag {
	type Error = Exception;

	fn try_from(text: String) -> Result<Self, Self::Error> {
		if text.trim().is_empty() {
			Err(Exception::IllegalArgument("Tags can't have an empty text.".to_string()))
		} else {
			Ok(Self { text })
		}
	}
}

impl Tag {
	pub fn set_from_text(text: &str) -> Result<BTreeSet<Tag>, Exception> {
		lazy_static! {
			static ref TAG_REGEX: Regex = Regex::new("^\\[|\\]$").unwrap();
		}

		TAG_REGEX
			.replace_all(text, "")
			.split(",")
			.map(str::trim)
			.filter(|tag| !tag.is_empty())
			.map(ToString::to_string)
			.map(Tag::try_from)
			.collect()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	#[allow(non_snake_case)]
	fn empty_element_array__empty_tag() {
		let tags_text = "[ ]";
		let expected_tags = BTreeSet::new();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn single_element_array__single_tag() {
		let tags_text = "[$TAG]";
		let expected_tags: BTreeSet<_> = vec!["$TAG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn multiple_elements_array__multiple_tags() {
		let tags_text = "[$TAG,$TOG,$TUG]";
		let expected_tags: BTreeSet<_> = vec!["$TAG", "$TOG", "$TUG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn multiple_elements_array_with_spaces__multiple_tags_without_spaces() {
		let tags_text = "[$TAG ,  $TOG , $TUG  ]";
		let expected_tags: BTreeSet<_> = vec!["$TAG", "$TOG", "$TUG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn multiple_elements_array_with_just_spaces_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,  , $TUG  ]";
		let expected_tags: BTreeSet<_> = vec!["$TAG", "$TUG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn multiple_elements_array_with_empty_tag__empty_tag_is_ignored() {
		let tags_text = "[$TAG ,, $TUG  ]";
		let expected_tags: BTreeSet<_> = vec!["$TAG", "$TUG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}

	#[test]
	#[allow(non_snake_case)]
	fn multiple_elements_array_duplicate_tags__duplicate_tag_is_ignored() {
		let tags_text = "[$TAG, $TAG]";
		let expected_tags: BTreeSet<_> = vec!["$TAG"]
			.into_iter()
			.map(ToString::to_string)
			.map(|text| Tag { text })
			.collect();

		let tags = Tag::set_from_text(tags_text).unwrap();
		assert_eq!(expected_tags, tags);
	}
}
