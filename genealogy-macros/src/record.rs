use crate::attributes::{AttributeProperties, Attributes};
use crate::record::derives::Derives;
use crate::record::fields::RecordFields;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_quote, ItemStruct, Lit};
use syn::{Attribute, Ident, Visibility};

mod derives;
mod field;
mod fields;

pub fn record(record_parameters: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
	let mut build_constructor = true;
	let mut derives = Derives::default();
	if !record_parameters.is_empty() {
		let record_attributes = syn::parse2::<AttributeProperties>(record_parameters)?;

		for (property, value) in record_attributes.properties {
			let name = property.to_string();
			match (name.as_str(), value) {
				("constructor", Lit::Bool(boolean)) => {
					build_constructor = boolean.value;
				}
				("equals", Lit::Bool(boolean)) => {
					derives.equals = boolean.value;
				}
				("hash", Lit::Bool(boolean)) => {
					derives.hash = boolean.value;
				}
				_ => {
					return Err(syn::Error::new(
						property.span(),
						r#"Invalid attribute property for #[record], allowed are "constructor", "equals" and "hash" with a boolean value."#,
					));
				}
			}
		}
	}

	let ItemStruct {
		attrs,
		vis,
		struct_token: _,
		ident,
		generics,
		fields,
		semi_token: _,
	} = syn::parse2::<ItemStruct>(item)?;

	if generics.gt_token.is_some() || generics.lt_token.is_some() || generics.where_clause.is_some() {
		return Err(syn::Error::new(
			generics.span(),
			"#[record] currently doesn't work with generics.",
		));
	}

	let mut required_traits = derives.trait_paths();
	required_traits.push(parse_quote!(::std::fmt::Display));

	let fields = RecordFields::new(fields, vis.clone())?;
	let mut tokens = TokenStream::new();
	fields.to_type_assertions(&required_traits, &mut tokens);

	let record = Record {
		attributes: attrs.into(),
		visibility: vis,
		name: ident,
		fields,
		build_constructor,
		derives,
	};

	record.to_tokens(&mut tokens);

	Ok(tokens)
}

struct Record {
	attributes: Attributes,
	visibility: Visibility,
	name: Ident,
	fields: RecordFields,
	build_constructor: bool,
	derives: Derives,
}

impl ToTokens for Record {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Self {
			attributes,
			visibility,
			name,
			fields,
			build_constructor,
			derives,
		} = self;

		let mut items = TokenStream::new();
		fields.to_items(&mut items);

		let constructor = if *build_constructor {
			let mut parameters = TokenStream::new();
			fields.to_parameters(&mut parameters);
			let mut initializers = TokenStream::new();
			fields.to_initializers(&mut initializers);
			quote! {
				#visibility fn new(#parameters) -> Self {
					Self {
						#initializers
					}
				}
			}
		} else {
			quote!()
		};

		let mut accessors = TokenStream::new();
		fields.to_accessors(&mut accessors);

		let mut display = TokenStream::new();
		fields.to_display_implementation(&self.name, &mut display);

		tokens.extend(quote! {
			#derives
			#attributes
			#visibility struct #name {
				#items
			}

			impl #name {
				#constructor

				#accessors
			}

			#display
		});
	}
}

fn is_omit(attribute: &Attribute) -> bool {
	let path = attribute
		.path
		.segments
		.iter()
		.map(|segment| &segment.ident)
		.collect::<Vec<_>>();
	match path.as_slice() {
		[identifier] => *identifier == "omit",
		_ => false,
	}
}
