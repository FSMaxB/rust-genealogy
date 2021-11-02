use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub struct Derives {
	pub equals: bool,
	pub hash: bool,
}

impl Default for Derives {
	fn default() -> Self {
		Self {
			equals: true,
			hash: true,
		}
	}
}

impl ToTokens for Derives {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		if self.equals {
			quote!(#[derive(PartialEq, Eq)]).to_tokens(tokens);
		}

		if self.hash {
			quote!(#[derive(Hash)]).to_tokens(tokens);
		}

		quote!(#[derive(Clone, Debug)]).to_tokens(tokens)
	}
}
