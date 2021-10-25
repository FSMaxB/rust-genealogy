use crate::helpers::collector::Collector;
use crate::helpers::exception::Exception::{self, IllegalArgumentException};
use crate::helpers::files::Files;
use crate::helpers::list::List;
use crate::helpers::objects::Objects;
use crate::helpers::optional::Optional;
use crate::helpers::path::Path;
use crate::helpers::stream::Stream;
use crate::helpers::string::JString;
use crate::throw;
use std::fmt::Debug;

/// ```java
/// public final class Utils {
///
/// 	private Utils() {
/// 		// private constructor to prevent accidental instantiation of utility class
/// 	}
/// ```
///
/// The empty enum has the same effect as a private constructor, preventing instantiation.
pub enum Utils {}

impl Utils {
	/// ```java
	/// public static String removeOuterQuotationMarks(String string) {
	/// 	return string.replaceAll("^\"|\"$", "");
	/// }
	/// ```
	pub fn remove_outer_quotation_marks(string: JString) -> Result<JString, Exception> {
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
	pub fn unchecked_files_list(dir: Path) -> Result<Stream<Path>, Exception> {
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
	pub fn unchecked_files_write(path: Path, content: JString) -> Result<(), Exception> {
		Files::write(path, List::of([content]))
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
	pub fn unchecked_files_read_all_lines(file: Path) -> Result<List<JString>, Exception> {
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
	pub fn unchecked_files_lines(file: Path) -> Result<Stream<JString>, Exception> {
		Files::lines(file)
	}

	/// ```java
	/// @SuppressWarnings("unchecked")
	/// public static <ELEMENT> Stream<ELEMENT> concat(Stream<? extends ELEMENT>... streams) {
	/// 	return Stream.of(streams).flatMap(s -> s);
	/// }
	/// ```
	pub fn concat<Element>(streams: impl IntoIterator<Item = Stream<Element>> + 'static) -> Stream<Element>
	where
		Element: 'static,
	{
		Stream::of(streams).flat_map(|s| s)
	}

	/// ```java
	/// public static <ELEMENT> Collector<ELEMENT, ?, Optional<ELEMENT>> collectEqualElement() {
	/// 	return collectEqualElement(Objects::equals);
	/// }
	/// ```
	pub fn collect_equal_element<Element>() -> Collector<Element, Optional<Element>, Optional<Element>>
	where
		Element: Clone + Debug + PartialEq + 'static,
	{
		Self::collect_equal_element_with_predicate(Objects::equals)
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
		equals: impl Fn(Element, Element) -> bool + 'static,
	) -> Collector<Element, Optional<Element>, Optional<Element>>
	where
		Element: Clone + Debug + 'static,
	{
		Collector::of(
			|| Ok(Optional::empty()),
			move |left, right: Element| {
				if left.is_present() && !equals(left.get()?, right.clone()) {
					throw!(IllegalArgumentException(
						format!("Unequal elements in stream: {:?} vs {:?}", left.get()?, &right).into()
					));
				}
				left.set(right);
				Ok(())
			},
			Ok,
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
/// class UtilsTests {
/// ```
#[cfg(test)]
mod test {
	use super::*;
	use crate::text_parser_tests::{self, test_text_parser};

	///```java
	/// @Nested
	/// class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	struct QuotationTests;

	///```java
	/// @Nested
	/// class QuotationTests implements TextParserTests.QuotationTests {
	/// ```
	impl text_parser_tests::QuotationTests for QuotationTests {
		/// ```java
		/// @Override
		/// public String parseCreateExtract(String text) {
		/// 	return Utils.removeOuterQuotationMarks(text);
		/// }
		/// ```
		fn parse_create_extract(text: JString) -> Result<JString, Exception> {
			Utils::remove_outer_quotation_marks(text)
		}
	}

	#[test]
	fn quotation_tests() {
		test_text_parser::<QuotationTests>();
	}

	/// ```java
	/// @Nested
	/// class CollectEqualElement {
	/// ```
	#[allow(non_snake_case)]
	mod collect_equal_element {
		use crate::helpers::exception::Exception::IllegalArgumentException;
		use crate::helpers::optional::Optional;
		use crate::helpers::stream::Stream;
		use crate::helpers::test::assert_that;

		/// ```java
		/// @Test
		/// void emptyStream_emptyOptional() {
		/// 	Optional<Object> element = Stream
		/// 			.of()
		/// 			.collect(collectEqualElement());
		///
		/// 	assertThat(element).isEmpty();
		/// }
		/// ```
		#[test]
		pub(super) fn empty_stream__empty_optional() {
			let element: Optional<i32> = Stream::of([]).collect(collect_equal_element!()).unwrap();

			assert_that(element).is_empty();
		}

		/// ```java
		/// @Test
		/// void singleElementStream_optionalWithThatElement() {
		/// 	Optional<String> element = Stream
		/// 			.of("element")
		/// 			.collect(collectEqualElement());
		///
		/// 	assertThat(element).contains("element");
		/// }
		/// ```
		#[test]
		pub(super) fn single_element_stream__optional_with_that_element() {
			let element = Stream::of(["element"]).collect(collect_equal_element!()).unwrap();

			assert_that(element).contains("element");
		}

		/// ```java
		/// @Test
		///	void equalElementStream_optionalWithThatElement() {
		///		Optional<String> element = Stream
		///				.of("element", "element", "element")
		///				.collect(collectEqualElement());
		///
		///		assertThat(element).contains("element");
		///	}
		/// ```
		#[test]
		pub(super) fn equal_element_stream__optional_with_that_element() {
			let element = Stream::of(["element", "element", "element"])
				.collect(collect_equal_element!())
				.unwrap();

			assert_that(element).contains("element");
		}

		/// ```java
		/// @Test
		///	void nonEqualElementStream_throwsException() {
		///		Stream<String> stream = Stream.of("element", "other element");
		///
		///		assertThatThrownBy(() -> stream.collect(collectEqualElement()))
		///				.isInstanceOf(IllegalArgumentException.class);
		///	}
		/// ```
		#[test]
		pub(super) fn non_equal_element_stream__throws_exception() {
			let stream = Stream::of(["element", "other_element"]);

			assert_that(move || stream.collect(collect_equal_element!()))
				.throws()
				.and_satisfies(|exception| matches!(exception, IllegalArgumentException(_)))
		}
	}
}
