use crate::helpers::exception::Exception;
use crate::helpers::string::JString;
use chrono::NaiveDate;

pub mod article_factory;
pub mod post_factory;
mod raw_front_matter;
pub mod raw_post;
pub mod talk_factory;
pub mod video_factory;

pub fn parse_date(text: JString) -> Result<NaiveDate, Exception> {
	Ok(NaiveDate::parse_from_str(text.as_ref(), "%Y-%m-%d")?)
}
