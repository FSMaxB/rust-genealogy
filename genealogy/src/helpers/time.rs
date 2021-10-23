use crate::helpers::exception::Exception;
use crate::helpers::string::JString;
use chrono::NaiveDate;

pub type LocalDate = NaiveDate;

pub trait LocalDateExtension {
	fn today() -> LocalDate;
	fn of(year: i32, month: u8, day: u8) -> LocalDate;
	fn parse(string: JString) -> Result<LocalDate, Exception>;
}

impl LocalDateExtension for LocalDate {
	fn today() -> LocalDate {
		chrono::offset::Local::today().naive_local()
	}

	fn of(year: i32, month: u8, day: u8) -> LocalDate {
		NaiveDate::from_ymd(year, month as u32, day as u32)
	}

	fn parse(text: JString) -> Result<LocalDate, Exception> {
		Ok(NaiveDate::parse_from_str(text.as_ref(), "%Y-%m-%d")?)
	}
}
