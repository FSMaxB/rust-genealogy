use chrono::NaiveDate;

pub type LocalDate = NaiveDate;

pub trait LocalDateExtension {
	fn today() -> LocalDate;
}

impl LocalDateExtension for LocalDate {
	fn today() -> LocalDate {
		chrono::offset::Local::today().naive_local()
	}
}