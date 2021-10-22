use crate::helpers::string::JString;
use crate::post::tag::Tag;
use std::collections::HashSet;

pub fn hash_set_of_tags(texts: &[&str]) -> HashSet<Tag> {
	texts
		.iter()
		.copied()
		.map(JString::from)
		.map(|text| Tag { text })
		.collect()
}
