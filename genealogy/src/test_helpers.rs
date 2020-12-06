use crate::post::tag::Tag;
use std::collections::BTreeSet;

pub fn btree_set_of_tags(texts: &[&str]) -> BTreeSet<Tag> {
	texts
		.iter()
		.copied()
		.map(str::to_owned)
		.map(|text| Tag { text })
		.collect()
}
