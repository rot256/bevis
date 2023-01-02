use std::vec;

use proc_macro;
use proc_macro2::{Ident, TokenStream};

use quote::{format_ident, quote};

use syn::spanned::Spanned;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
    Token,
};

mod absorb;
mod challenge;
mod transcript;

// derive of Absorb
use absorb::impl_absorb;

// derive of Transcript
use transcript::impl_transcript;

// derieve of Challenge
use challenge::impl_challenge;

struct SpongeType(Vec<Ident>);

impl Parse for SpongeType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let content;
        syn::parenthesized!(content in input);

        let mut types = vec![content.parse()?];

        while !content.is_empty() {
            content.parse::<Token![,]>()?;
            types.push(content.parse()?);
        }

        Ok(Self(types))
    }
}

#[proc_macro_derive(Tx)]
pub fn derive_transcript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_transcript(input)
}

#[proc_macro_derive(Absorb)]
pub fn derive_absorb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_absorb(&ast)
}

#[proc_macro_derive(Challenge)]
pub fn derive_challenge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    impl_challenge(&ast)
}
