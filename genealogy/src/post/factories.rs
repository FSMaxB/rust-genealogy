use crate::helpers::exception::Exception;
use chrono::NaiveDate;

pub mod article_factory;
mod raw_front_matter;
pub mod raw_post;
pub mod video_factory;

pub fn parse_date(text: &str) -> Result<NaiveDate, Exception> {
	Ok(NaiveDate::parse_from_str(text, "%Y-%m-%d")?)
}
