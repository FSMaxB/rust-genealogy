use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Attribute, Ident, Lit, Token};

/// A map of comma separated key-value properties in an attribute.
/// `#[attribute(text = "value", number = 42)]`
pub struct AttributeProperties {
	pub properties: HashMap<Ident, Lit>,
}

impl Parse for AttributeProperties {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let properties = Punctuated::<AttributeProperty, Token!(,)>::parse_terminated(input)?
			.into_iter()
			.map(|property| (property.name, property.value))
			.collect();
		Ok(Self { properties })
	}
}

pub struct AttributeProperty {
	pub name: Ident,
	pub value: Lit,
}

impl Parse for AttributeProperty {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let name = input.parse::<Ident>()?;
		input.parse::<Token!(=)>()?;
		let value = input.parse::<Lit>()?;
		Ok(Self { name, value })
	}
}

pub struct Attributes(Vec<Attribute>);

impl From<Vec<Attribute>> for Attributes {
	fn from(attributes: Vec<Attribute>) -> Self {
		Self(attributes)
	}
}

impl ToTokens for Attributes {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		for attribute in &self.0 {
			attribute.to_tokens(tokens);
		}
	}
}
