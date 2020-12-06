use crate::java_replicas::exception::Exception;
use chrono::NaiveDate;

mod article_factory;
mod post_factory;
mod raw_front_matter;
mod raw_post;
mod talk_factory;
mod video_factory;

pub(self) fn parse_date(text: &str) -> Result<NaiveDate, Exception> {
	Ok(NaiveDate::parse_from_str(text, "%Y-%m-%d")?)
}
