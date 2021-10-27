#![allow(clippy::tabs_in_doc_comments)]

use proc_macro::TokenStream;

mod record;

#[proc_macro_attribute]
pub fn record(attribute: TokenStream, item: TokenStream) -> TokenStream {
	record::record(attribute.into(), item.into())
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
