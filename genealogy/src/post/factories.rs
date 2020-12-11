use crate::helpers::exception::Exception;
use chrono::NaiveDate;

pub mod article_factory;
mod raw_front_matter;
mod raw_post;
pub mod talk_factory;
pub mod video_factory;

pub(self) fn parse_date(text: &str) -> Result<NaiveDate, Exception> {
	Ok(NaiveDate::parse_from_str(text, "%Y-%m-%d")?)
}
