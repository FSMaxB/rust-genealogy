#![allow(clippy::tabs_in_doc_comments)]
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Attribute, ExprField, Field, Fields, FieldsNamed, Ident, Visibility};
use syn::{parse_quote, ItemStruct, Lit, Token, Type};

/// Takes a struct and implements both a constructor and accessors, similarly to a Java record.
///
/// ```
/// //use genealogy_macros::record;
///
/// //#[record]
/// //struct Greetings {
/// //	text1: &'static str,
/// //	text2: String,
/// //}
///
/// //let greeting = Greetings::new("hello", String::from("world"));
/// //println!("{} {}", greeting.text1(), greeting.text2());
/// ```
// TODO: Automatically generate derives.
#[proc_macro_attribute]
pub fn record(record_parameters: TokenStream, item: TokenStream) -> TokenStream {
	let mut build_constructor = true;
	if !record_parameters.is_empty() {
		let record_attributes = parse_macro_input!(record_parameters as AttributeProperties);

		for (property, value) in record_attributes.properties {
			let name = property.to_string();
			match (name.as_str(), value) {
				("constructor", Lit::Bool(boolean)) => {
					build_constructor = boolean.value;
				}
				_ => {
					return syn::Error::new(property.span(), "Invalid attribute property for #[record]")
						.into_compile_error()
						.into()
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
	} = parse_macro_input!(item as ItemStruct);

	if generics.gt_token.is_some() || generics.lt_token.is_some() || generics.where_clause.is_some() {
		// FIXME: Make it work with generics
		return syn::Error::new(generics.span(), "#[record] currently doesn't work with generics.")
			.into_compile_error()
			.into();
	}

	let fields = match RecordFields::new(fields, vis.clone()) {
		Ok(fields) => fields,
		Err(error) => {
			return error.into_compile_error().into();
		}
	};

	let record = Record {
		attributes: attrs.into(),
		visibility: vis,
		name: ident,
		fields,
		build_constructor,
	};

	let mut tokens = TokenStream2::new();
	record.to_tokens(&mut tokens);

	tokens.into()
}

struct AttributeProperties {
	properties: HashMap<Ident, Lit>,
}

impl Parse for AttributeProperties {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut properties = HashMap::new();
		while !input.is_empty() {
			let key = input.parse::<Ident>()?;
			input.parse::<Token!(=)>()?;
			let value = input.parse::<Lit>()?;
			properties.insert(key, value);
		}
		Ok(Self { properties })
	}
}

struct Record {
	attributes: Attributes,
	visibility: Visibility,
	name: Ident,
	fields: RecordFields,
	build_constructor: bool,
}

impl ToTokens for Record {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let Self {
			attributes,
			visibility,
			name,
			fields,
			build_constructor,
		} = self;

		let mut items = TokenStream2::new();
		fields.to_items(&mut items);

		let constructor = if *build_constructor {
			let mut parameters = TokenStream2::new();
			fields.to_parameters(&mut parameters);
			let mut initializers = TokenStream2::new();
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

		let mut accessors = TokenStream2::new();
		fields.to_accessors(&mut accessors);

		let mut display = TokenStream2::new();
		fields.to_display_implementation(&self.name, &mut display);

		tokens.extend(quote! {
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

struct RecordFields(Vec<RecordField>);

impl RecordFields {
	fn new(fields: Fields, record_visibility: Visibility) -> syn::Result<RecordFields> {
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

	fn to_items(&self, tokens: &mut TokenStream2) {
		for field in &self.0 {
			field.to_item(tokens);
		}
	}

	fn to_parameters(&self, tokens: &mut TokenStream2) {
		for field in &self.0 {
			field.to_parameter(tokens);
		}
	}

	fn to_initializers(&self, tokens: &mut TokenStream2) {
		for field in &self.0 {
			field.to_initializer(tokens);
		}
	}

	fn to_accessors(&self, tokens: &mut TokenStream2) {
		for field in &self.0 {
			field.to_accessor(tokens);
		}
	}

	fn to_display_implementation(&self, record_name: &Ident, tokens: &mut TokenStream2) {
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
}

struct RecordField {
	attributes: Attributes,
	visibility: Visibility,
	name: Ident,
	r#type: Type,
	omit: bool,
}

struct Attributes(Vec<Attribute>);

impl From<Vec<Attribute>> for Attributes {
	fn from(attributes: Vec<Attribute>) -> Self {
		Self(attributes)
	}
}

impl ToTokens for Attributes {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		for attribute in &self.0 {
			attribute.to_tokens(tokens);
		}
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

impl RecordField {
	fn new(field: Field, visibility: Visibility) -> syn::Result<RecordField> {
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

	fn to_item(&self, tokens: &mut TokenStream2) {
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

	fn to_parameter(&self, tokens: &mut TokenStream2) {
		let Self { name, r#type, .. } = self;
		tokens.extend(quote! {
			#name: #r#type,
		});
	}

	fn to_initializer(&self, tokens: &mut TokenStream2) {
		let Self { name, .. } = self;
		tokens.extend(quote!(#name,));
	}

	fn to_accessor(&self, tokens: &mut TokenStream2) {
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
