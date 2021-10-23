use chrono::NaiveDate;

pub type LocalDate = NaiveDate;

pub trait LocalDateExtension {
	fn today() -> LocalDate;
	fn of(year: i32, month: u8, day: u8) -> LocalDate;
}

impl LocalDateExtension for LocalDate {
	fn today() -> LocalDate {
		chrono::offset::Local::today().naive_local()
	}

	fn of(year: i32, month: u8, day: u8) -> LocalDate {
		NaiveDate::from_ymd(year, month as u32, day as u32)
	}
}
