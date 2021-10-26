#![allow(clippy::tabs_in_doc_comments)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::iter::once;
use syn::parse_macro_input;
use syn::ItemStruct;

/// Takes a struct and implements both a constructor and accessors, similarly to a Java record.
///
/// ```
/// use genealogy_macros::record;
///
/// #[record]
/// struct Greetings {
/// 	text1: &'static str,
/// 	text2: String,
/// }
///
/// let greeting = Greetings::new("hello", String::from("world"));
/// println!("{} {}", greeting.text1(), greeting.text2());
/// ```
// TODO: Allow conditional constructor generation.
// TODO: Automatically generate derives.
// TODO: Allow overriding methods
#[proc_macro_attribute]
pub fn record(_attribute: TokenStream, item: TokenStream) -> TokenStream {
	let record = parse_macro_input!(item as ItemStruct);
	let visibility = &record.vis;

	let mut constructor_arguments = TokenStream2::new();
	let mut names = TokenStream2::new();
	let mut accessors = TokenStream2::new();
	for (index, field) in record.fields.iter().enumerate() {
		let name = field.ident.as_ref().unwrap();
		let field_type = &field.ty;
		constructor_arguments.extend(once(quote!(#name: #field_type)));
		names.extend(quote!(#name));
		accessors.extend(quote! {
			#visibility fn #name(&self) -> #field_type {
				::std::clone::Clone::clone(&self.#name)
			}
		});
		if index < (record.fields.len() - 1) {
			// not the last field
			constructor_arguments.extend(quote!(,));
			names.extend(quote!(,))
		}
	}

	let name = &record.ident;
	let tokens = quote! {
		#record

		impl #name {
			#visibility fn new(#constructor_arguments) -> Self {
				Self {
					#names
				}
			}

			#accessors
		}
	};
	tokens.into()
}
