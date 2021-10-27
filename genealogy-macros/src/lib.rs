#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;

mod record;

/// Takes a struct and implements both a constructor and accessors, similarly to a Java record.
///
/// ```
/// use genealogy_macros::record;
///
/// #[record]
/// struct Record {
/// 	text: &'static str,
/// 	number: i32,
/// }
///
/// // Automatically generates a constructor
/// let record = Record::new("hello", 42);
/// // And accessor functions
/// assert_eq!("hello", record.text());
/// assert_eq!(42, record.number());
///
/// // Automatically implements PartialEq, Eq, Hash, Debug, Clone and Display
/// let clone = record.clone();
/// assert_eq!(clone, record);
/// assert_eq!("Record[text=hello, number=42]", record.to_string());
/// assert_eq!(r#"Record { text: "hello", number: 42 }"#, format!("{:?}", record));
/// let mut set = std::collections::HashSet::new();
/// set.insert(record);
/// ```
///
/// You can also optionally disable generating some of the implementations:
/// `#[record(constructor = false, equals = false, hash = false)]`
///
/// Disable generating accessors by marking the respective field with `#[omit]`, this allows
/// overriding the implementation of an accessor.
/// ```
/// use genealogy_macros::record;
///
/// #[record(constructor = false)]
/// struct Record {
/// 	#[omit]
/// 	text: String,
/// }
///
/// impl Record {
/// 	// by specifying `constructor = false`, you can write your own
/// 	pub fn new(text: &'static str) -> Self {
/// 		Self {
/// 			text: text.into(),
/// 		}
/// 	}
///
/// 	// `#[omit]` allows providing your own accessor function
/// 	pub fn text(&self) -> &str {
/// 		&self.text
/// 	}
/// }
/// ```
#[proc_macro_attribute]
pub fn record(attribute: TokenStream, item: TokenStream) -> TokenStream {
	record::record(attribute.into(), item.into())
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
