use crate::helpers::collector::Collector;
use crate::helpers::exception::Exception::{self, IllegalArgumentException};
use crate::helpers::files::Files;
use crate::helpers::stream::Stream;
use crate::helpers::string_extensions::StringExtensions;
use crate::{list_of, throw};
use std::convert::identity;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// ```java
/// public final class Utils {
///
/// 	private Utils() {
/// 		// private constructor to prevent accidental instantiation of utility class
/// 	}
/// ```
///
/// Note that making the constructor private isn't really possible, so I'm leaving it out
pub struct Utils;

impl Utils {
	/// ```java
	/// public static String removeOuterQuotationMarks(String string) {
	/// 	return string.replaceAll("^\"|\"$", "");
	/// }
	/// ```
	pub fn remove_outer_quotation_marks(string: &str) -> Result<String, Exception> {
		string.replace_all(r#"^"|"$"#, "")
	}

	/// ```java
	/// public static Stream<Path> uncheckedFilesList(Path dir) {
	/// 	try {
	/// 		return Files.list(dir);
	/// 	} catch (IOException ex) {
	/// 		throw new UncheckedIOException(ex);
	/// 	}
	/// }
	/// ```
	pub fn unchecked_files_list(dir: &Path) -> Result<Stream<PathBuf>, Exception> {
		Files::list(dir)
	}

	/// ```java
	/// public static <T> void uncheckedFilesWrite(Path path, String content) {
	/// 	try {
	/// 		Files.write(path, List.of(content));
	/// 	} catch (IOException ex) {
	/// 		throw new UncheckedIOException(ex);
	/// 	}
	/// }
	/// ```
	pub fn unchecked_files_write(path: &Path, content: &str) -> Result<(), Exception> {
		Files::write(path, list_of!(content))
	}

	/// ```java
	/// public static List<String> uncheckedFilesReadAllLines(Path file) {
	/// 	try {
	/// 		return Files.readAllLines(file);
	/// 	} catch (IOException ex) {
	/// 		throw new UncheckedIOException(ex);
	/// 	}
	/// }
	/// ```
	pub fn unchecked_files_read_all_lines(file: &Path) -> Result<Vec<String>, Exception> {
		Files::read_all_lines(file)
	}

	/// ```java
	/// public static Stream<String> uncheckedFilesLines(Path file) {
	/// 	try {
	/// 		return Files.lines(file);
	/// 	} catch (IOException ex) {
	/// 		throw new UncheckedIOException(ex);
	/// 	}
	/// }
	/// ```
	pub fn unchecked_files_lines(file: &Path) -> Result<Stream<String>, Exception> {
		Files::lines(file)
	}

	/// ```java
	/// public static <ELEMENT> Collector<ELEMENT, ?, Optional<ELEMENT>> collectEqualElement() {
	/// 	return collectEqualElement(Objects::equals);
	/// }
	/// ```
	pub fn collect_equal_element<Element>() -> Collector<Element, Option<Element>, Option<Element>>
	where
		Element: Display + PartialEq + 'static,
	{
		Self::collect_equal_element_with_predicate(Element::eq)
	}

	/// ```java
	/// public static <ELEMENT> Collector<ELEMENT, ?, Optional<ELEMENT>> collectEqualElement(
	/// 		BiPredicate<ELEMENT, ELEMENT> equals) {
	/// 	return Collector.of(
	/// 			AtomicReference::new,
	/// 			(AtomicReference<ELEMENT> left, ELEMENT right) -> {
	/// 				if (left.get() != null && !equals.test(left.get(), right))
	/// 					throw new IllegalArgumentException(
	/// 							"Unequal elements in stream: %s vs %s".formatted(left.get(), right));
	/// 				left.set(right);
	/// 			},
	/// 			(AtomicReference<ELEMENT> left, AtomicReference<ELEMENT> right) -> {
	/// 				if (left.get() != null && right.get() != null && !equals.test(left.get(), right.get()))
	/// 					throw new IllegalArgumentException(
	/// 							"Unequal elements in stream: %s vs %s".formatted(left.get(), right.get()));
	/// 				return left.get() != null ? left : right;
	/// 			},
	/// 			reference -> Optional.ofNullable(reference.get())
	/// 	);
	/// }
	/// ```
	pub fn collect_equal_element_with_predicate<Element>(
		equals: impl Fn(&Element, &Element) -> bool + 'static,
	) -> Collector<Element, Option<Element>, Option<Element>>
	where
		Element: Display + 'static,
	{
		Collector::of(
			Option::default,
			move |left, right| {
				if left.is_some() && !equals(left.as_ref().unwrap(), &right) {
					throw!(IllegalArgumentException(format!(
						"Unequal elements in stream: {} vs {}",
						left.as_ref().unwrap(),
						&right
					)));
				}
				left.replace(right);
				Ok(())
			},
			identity,
		)
	}
}

/// Macro implementation to simulate overloading
#[macro_export]
macro_rules! collect_equal_element {
	() => {
		crate::utils::Utils::collect_equal_element()
	};
	($predicate: expr) => {
		crate::utils::Utils::collect_equal_element_with_predicate($predicate)
	};
}

/// ```java
/// @SuppressWarnings("unchecked")
/// public static <ELEMENT> Stream<ELEMENT> concat(Stream<? extends ELEMENT>... streams) {
/// 	return Stream.of(streams).flatMap(s -> s);
/// }
/// ```
///
/// Implemented as a macro to simulate variadic function.
#[macro_export]
macro_rules! concat {
	() => {
		crate::stream_of!().flat_map()
	};
	($($element: expr), + $(,) ?) => {
		crate::stream_of!($($element),+).flat_map()
	};
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::test_text_parser;

	#[test]
	fn quotation_tests() {
		test_text_parser(Utils::remove_outer_quotation_marks);
	}
}
