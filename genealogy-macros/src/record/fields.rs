use crate::record::field::RecordField;
use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_quote, ExprField, Fields, FieldsNamed, Ident, Path, Token, Visibility};

pub struct RecordFields(Vec<RecordField>);

impl RecordFields {
	pub fn new(fields: Fields, record_visibility: Visibility) -> syn::Result<RecordFields> {
		let fields = match fields {
			Fields::Named(FieldsNamed { named, .. }) => named,
			_ => {
				return Err(syn::Error::new(
					fields.span(),
					"#[record] is only allowed for structs with named fields.",
				))
			}
		};
		let record_fields = fields
			.into_iter()
			.map(move |field| RecordField::new(field, record_visibility.clone()))
			.collect::<Result<_, _>>()?;
		Ok(RecordFields(record_fields))
	}

	pub fn to_items(&self, tokens: &mut TokenStream) {
		for field in &self.0 {
			field.to_item(tokens);
		}
	}

	pub fn to_parameters(&self, tokens: &mut TokenStream) {
		for field in &self.0 {
			field.to_parameter(tokens);
		}
	}

	pub fn to_initializers(&self, tokens: &mut TokenStream) {
		for field in &self.0 {
			field.to_initializer(tokens);
		}
	}

	pub fn to_accessors(&self, tokens: &mut TokenStream) {
		for field in &self.0 {
			field.to_accessor(tokens);
		}
	}

	pub fn to_display_implementation(&self, record_name: &Ident, tokens: &mut TokenStream) {
		let field_format = self
			.0
			.iter()
			.map(|field| format!("{}={{}}", field.name))
			.collect::<Vec<_>>();
		let format_string = format!("{}[{}]", record_name, field_format.join(", "));

		let format_arguments = self
			.0
			.iter()
			.map(|field| -> ExprField {
				let name = &field.name;
				parse_quote!(self.#name)
			})
			.collect::<Punctuated<_, Token!(,)>>();

		tokens.extend(quote! {
			impl ::std::fmt::Display for #record_name {
				fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
					::std::write!(formatter, #format_string, #format_arguments)
				}
			}
		})
	}

	pub fn to_type_assertions(&self, trait_paths: &[Path], tokens: &mut TokenStream) {
		for field in &self.0 {
			field.to_type_assertion(trait_paths, tokens);
		}
	}
}
