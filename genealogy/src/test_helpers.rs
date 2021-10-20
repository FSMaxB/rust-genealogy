use crate::post::tag::Tag;
use std::collections::HashSet;

pub fn hash_set_of_tags(texts: &[&str]) -> HashSet<Tag> {
	texts
		.iter()
		.copied()
		.map(str::to_owned)
		.map(|text| Tag { text })
		.collect()
}
