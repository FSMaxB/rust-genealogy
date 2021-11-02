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

	let record: Record = Record::new();
	assert_eq!("Record[]", record.to_string());
}

#[test]
fn record_with_one_field() {
	#[record]
	struct Record {
		value: usize,
	}

	let record = Record::new(42);
	assert_eq!(42, record.value());
	assert_eq!("Record[value=42]", record.to_string())
}

#[test]
fn record_with_two_fields() {
	#[record]
	struct Record {
		value: usize,
		text: &'static str,
	}

	let record = Record::new(42, "hello");
	assert_eq!(42, record.value());
	assert_eq!(42, record.value);
	assert_eq!("hello", record.text());
	assert_eq!("hello", record.text);
	assert_eq!("Record[value=42, text=hello]", record.to_string());
	assert_eq!(r#"Record { value: 42, text: "hello" }"#, format!("{:?}", record));
	assert_eq!(
		r#"Record {
    value: 42,
    text: "hello",
}"#,
		format!("{:#?}", record)
	);
}

#[test]
fn record_with_two_fields_all_derives_manually_enabled() {
	#[record(constructor = true, equals = true, hash = true)]
	struct Record {
		value: usize,
		text: &'static str,
	}

	let record = Record::new(42, "hello");
	assert_eq!(42, record.value());
	assert_eq!(42, record.value);
	assert_eq!("hello", record.text());
	assert_eq!("hello", record.text);
	assert_eq!("Record[value=42, text=hello]", record.to_string());
	assert_eq!(r#"Record { value: 42, text: "hello" }"#, format!("{:?}", record));
	assert_eq!(
		r#"Record {
    value: 42,
    text: "hello",
}"#,
		format!("{:#?}", record)
	);
}

#[test]
fn record_with_derive() {
	#[record]
	struct Record {}

	let record = Record::new();
	#[allow(clippy::redundant_clone)]
	let cloned = record.clone();
	assert_eq!("Record[]", cloned.to_string());
}

#[test]
fn record_without_constructor() {
	#[record(constructor = false)]
	struct Record {}

	impl Record {
		pub fn new() -> Self {
			Self {}
		}
	}

	let _record = Record::new();
}

#[test]
fn record_with_omitted_accessor() {
	#[record]
	struct Record {
		#[omit]
		number: usize,
	}

	impl Record {
		pub fn number(&self) -> usize {
			self.number
		}
	}

	let record = Record::new(42);
	assert_eq!(42, record.number);
	assert_eq!(42, record.number());
}

#[test]
fn public_record_has_public_methods() {
	mod inner {
		use super::*;

		#[record]
		pub struct Record {
			number: usize,
		}
	}

	let record = inner::Record::new(42);
	assert_eq!(42, record.number());
}

#[test]
fn record_fails_to_compile_if_invalid() {
	let test = trybuild::TestCases::new();
	test.compile_fail("tests/ui/*.rs");
}
