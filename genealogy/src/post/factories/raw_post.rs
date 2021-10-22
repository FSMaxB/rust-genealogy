use crate::post::content::Content;
use crate::post::factories::raw_front_matter::RawFrontMatter;

/// ```java
/// class RawPost {
/// 	private final RawFrontMatter frontMatter;
/// 	private final Content content;
/// ````
pub(super) struct RawPost {
	front_matter: RawFrontMatter,
	content: Content,
}

impl RawPost {
	/// ```java
	/// RawPost(RawFrontMatter frontMatter, Content content) {
	/// 	this.frontMatter = frontMatter;
	/// this.content = content;
	/// }
	/// ```
	pub(super) fn new(front_matter: RawFrontMatter, content: Content) -> Self {
		Self { front_matter, content }
	}

	/// ```java
	/// public RawFrontMatter frontMatter() {
	///		return frontMatter;
	///	}
	/// ```
	pub fn front_matter(&self) -> &RawFrontMatter {
		&self.front_matter
	}

	/// ```java
	/// public Content content() {
	///		return content;
	///	}
	/// ```
	/// Note: Consumes the [`RawPost`] because the [`Content`] can only be used
	/// once destructively anyways.
	pub fn content(self) -> Content {
		self.content
	}
}
