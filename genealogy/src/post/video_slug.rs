use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string_extensions::StringExtensions;
use crate::throw;

/// ```java
/// public record VideoSlug(String value) implements Comparable<VideoSlug> {
/// 	@Override
///		public int compareTo(VideoSlug right) {
///			return this.value.compareTo(right.value);
///		}
/// ```
///
/// compareTo is automatically implemented by the PartialOrd and Ord derives
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSlug {
	pub value: String,
}

impl VideoSlug {
	/// ```java
	/// public VideoSlug {
	///		requireNonNull(value);
	///		if (value.isBlank())
	///			throw new IllegalArgumentException("Slugs can't have an empty value.");
	///	}
	/// ```
	pub fn new(value: String) -> Result<VideoSlug, Exception> {
		if value.is_blank() {
			throw!(IllegalArgumentException(
				"VideoSlugs can't have an empty value.".to_string()
			));
		}

		Ok(VideoSlug { value })
	}
}

/// No original Java code here, but identical to the slug tests.
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;

	#[test]
	fn empty_text__exception() {
		assert_that(|| VideoSlug::new("".to_string()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
