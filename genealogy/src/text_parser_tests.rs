use crate::helpers::exception::Exception;

pub fn test_text_parser(parser: impl Fn(&str) -> Result<String, Exception>) {
	struct Test {
		name: &'static str,
		input: &'static str,
		expected: &'static str,
	}

	let tests = [
		Test {
			name: "createFromStringWithoutQuotationMarks_noChange",
			input: "A cool blog post",
			expected: "A cool blog post",
		},
		Test {
			name: "createFromStringWithQuotationMarks_quotationMarksRemoved",
			input: "\"A cool blog post\"",
			expected: "A cool blog post",
		},
		Test {
			name: "createFromStringWithInnerQuotationMarks_onlyOuterQuotationMarksRemoved",
			input: "\"\"A cool blog post\" he said\"",
			expected: "\"A cool blog post\" he said",
		},
	];

	for Test { name, input, expected } in tests.iter() {
		assert_eq!(expected, &parser(input).unwrap(), "Test {} failed.", name);
	}
}
