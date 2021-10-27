use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Ident, Lit, Token};

/// A map of comma separated key-value properties in an attribute.
/// `#[attribute(text = "value", number = 42)]`
pub(crate) struct AttributeProperties {
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

pub(crate) struct AttributeProperty {
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
