use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Path};

pub struct Derives {
	pub equals: bool,
	pub hash: bool,
}

impl Derives {
	pub fn trait_paths(&self) -> Vec<Path> {
		let mut paths: Vec<Path> = vec![parse_quote!(::std::clone::Clone), parse_quote!(::std::fmt::Debug)];
		if self.equals {
			paths.push(parse_quote!(::std::cmp::PartialEq));
			paths.push(parse_quote!(::std::cmp::Eq));
		}

		if self.hash {
			paths.push(parse_quote!(::std::hash::Hash))
		}

		paths
	}
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
		for trait_path in self.trait_paths() {
			tokens.extend(quote! {
				#[derive(#trait_path)]
			})
		}
	}
}
