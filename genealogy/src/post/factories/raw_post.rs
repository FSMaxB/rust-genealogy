use crate::post::content::Content;
use crate::post::factories::raw_front_matter::RawFrontMatter;

#[allow(dead_code)]
pub struct RawPost {
	// NOTE: Using `pub` here instead of getters, because Java doesn't provide
	// any more privacy anyways since you can still modify them via the reference.
	// This also means we don't need a constructor!
	pub front_matter: RawFrontMatter,
	pub content: Content,
}
