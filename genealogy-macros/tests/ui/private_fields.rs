use genealogy_macros::record;

mod sub_module {
	use super::*;

	#[record]
	pub struct Record {
		number: i32,
	}
}

pub fn main() {
	let record = sub_module::Record::new(42);
	record.number;
}
