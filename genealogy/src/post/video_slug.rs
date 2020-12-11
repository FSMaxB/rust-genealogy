use crate::helpers::exception::Exception;
use crate::helpers::exception::Exception::IllegalArgument;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSlug {
	pub value: String,
}

impl VideoSlug {
	pub fn from_value(value: String) -> Result<VideoSlug, Exception> {
		if value.trim().is_empty() {
			Err(IllegalArgument("VideoSlugs can't have an empty value.".to_string()))
		} else {
			Ok(VideoSlug { value })
		}
	}

	// NOTE: compareTo is already handled by the `PartialOrd` and `Ord` derives.
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn empty_text_exception() {
		assert!(matches!(VideoSlug::from_value("".to_string()), Err(IllegalArgument(_))))
	}
}
