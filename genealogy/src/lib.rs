#![allow(clippy::tabs_in_doc_comments)]
pub mod config;
pub mod genealogist;
pub mod genealogy;
pub mod helpers;
pub mod post;
pub mod process_details;
pub mod recommendation;
#[cfg(test)]
pub mod test_helpers;

/// ```java
/// public class TextParserTests {
/// ```
#[cfg(test)]
pub mod text_parser_tests;

pub mod utils;
