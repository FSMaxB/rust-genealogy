use crate::attributes::Attributes;
use crate::record::is_omit;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Field, Ident, Type, Visibility};

pub struct RecordField {
	pub attributes: Attributes,
	pub visibility: Visibility,
	pub name: Ident,
	pub r#type: Type,
	pub omit: bool,
}

impl RecordField {
	pub fn new(field: Field, visibility: Visibility) -> syn::Result<RecordField> {
		let field_span = field.span();
		let mut omit = false;
		let attributes = field
			.attrs
			.into_iter()
			.filter(|attribute| {
				if is_omit(attribute) {
					omit = true;
					false
				} else {
					true
				}
			})
			.collect::<Vec<_>>()
			.into();

		let name = match field.ident {
			Some(name) => name,
			None => {
				return Err(syn::Error::new(field_span, "#[record] doesn't support tuple structs"));
			}
		};
		match field.vis {
			Visibility::Inherited => {}
			visibility => {
				return Err(syn::Error::new(
					visibility.span(),
					"All fields in #[record] must be private",
				));
			}
		}

		Ok(Self {
			attributes,
			visibility,
			name,
			r#type: field.ty,
			omit,
		})
	}

	pub fn to_item(&self, tokens: &mut TokenStream) {
		let Self {
			attributes,
			name,
			r#type,
			..
		} = self;
		tokens.extend(quote! {
			#attributes
			#name: #r#type,
		})
	}

	pub fn to_parameter(&self, tokens: &mut TokenStream) {
		let Self { name, r#type, .. } = self;
		tokens.extend(quote! {
			#name: #r#type,
		});
	}

	pub fn to_initializer(&self, tokens: &mut TokenStream) {
		let Self { name, .. } = self;
		tokens.extend(quote!(#name,));
	}

	pub fn to_accessor(&self, tokens: &mut TokenStream) {
		let Self {
			visibility,
			name,
			r#type,
			omit,
			..
		} = self;

		if *omit {
			return;
		}

		tokens.extend(quote! {
			#visibility fn #name(&self) -> #r#type {
				::std::clone::Clone::clone(&self.#name)
			}
		});
	}
}
