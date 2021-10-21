use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgumentException;
use crate::helpers::string_extensions::StringExtensions;
use crate::throw;

/// ```java
/// public record Slug(String value) implements Comparable<Slug> {
///		@Override
///		public int compareTo(Slug right) {
///			return this.value.compareTo(right.value);
///		}
/// ```
///
/// compareTo is automatically implemented by the PartialOrd and Ord derives
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slug {
	pub value: String,
}

impl Slug {
	/// ```java
	/// public Slug {
	///		value = requireNonNull(value);
	///		if (value.isBlank())
	///			throw new IllegalArgumentException("Slugs can't have an empty value.");
	///	}
	/// ```
	pub fn new(value: String) -> Result<Slug, Exception> {
		if value.is_blank() {
			throw!(IllegalArgumentException("Slugs can't have an empty value.".to_string()));
		}

		Ok(Slug { value })
	}
}

/// ```java
/// class SlugTests {
/// ```
#[allow(non_snake_case)]
#[cfg(test)]
mod test {
	use super::*;
	use crate::helpers::test::assert_that;

	/// ```java
	/// @Test
	///	void emptyText_exception() {
	///		assertThatThrownBy(() -> new Slug("")).isInstanceOf(IllegalArgumentException.class);
	///	}
	/// ```
	#[test]
	fn empty_text__exception() {
		assert_that(|| Slug::new("".to_string()))
			.throws()
			.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)));
	}
}
