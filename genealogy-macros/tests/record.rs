use genealogy_macros::record;

#[test]
fn empty_record() {
	#[record]
	struct Record {}

	let _record: Record = Record::new();
}

#[test]
fn empty_record_with_explicit_constructor() {
	#[record(constructor = true)]
	struct Record {}

	let _record: Record = Record::new();
}

#[test]
fn record_with_one_attribute() {
	#[record]
	struct Record {
		value: usize,
	}

	let record = Record::new(42);
	assert_eq!(42, record.value());
}

#[test]
fn record_with_two_attribute() {
	#[record]
	struct Record {
		value: usize,
		text: &'static str,
	}

	let record = Record::new(42, "hello");
	assert_eq!(42, record.value());
	assert_eq!("hello", record.text);
}

#[test]
fn record_with_derive() {
	#[record]
	#[derive(Clone)]
	struct Record {}
}

// TODO: Test constructor = false
// TODO: Test accessors
// TODO: Test omitting accessors
// TODO: Test visibility
// TODO: Test Clone implementation of types
// TODO: Test compiler error with generics
// TODO: Test compiler error with tuple-structs and empty structs
