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

const NAME_ABSORB: &str = "absorb";
const NAME_CHALLENGE: &str = "challenge";

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

fn get_types(tr: &str, name: &str, ast: &syn::DeriveInput) -> Result<SpongeType, syn::Error> {
    let sponge_attr: Vec<_> = ast
        .attrs
        .iter()
        .filter(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == name)
        .collect();

    match &sponge_attr[..] {
        [attr] => {
            Ok(syn::parse2(attr.tokens.clone())?)
        }
        [] => {
            Err(
                syn::Error::new(
                    ast.span(),
                    format!(
                        "when deriving \"{tr}\" must provide the \"{name}\" attribute. e.g. \"#[{name}(u8)]\" or \"#[{name}(ff::SomeField)]\"", tr=tr, name=name)
                )
            )
        }
        _ => {
            Err(
                syn::Error::new(
                    ast.span(),
                    format!("too many of the \"{name}\" attribute provided (must be 1). Combine them, e.g. \"{name}(u8, ff::SomeField)\"", name=name)
                )
            )
        }
    }
}

#[proc_macro_derive(Tx)]
pub fn derive_transcript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_transcript(input)
}

#[proc_macro_derive(Absorb, attributes(absorb))]
pub fn derive_absorb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // parse the list of types for which to impl
    let types = match get_types("Absorb", NAME_ABSORB, &ast) {
        Ok(types) => types,
        Err(err) => return err.to_compile_error().into(),
    };

    // join all the implementations
    let mut result = proc_macro::TokenStream::new();
    for tt in types.0.iter() {
        result.extend(impl_absorb(tt, &ast).into_iter())
    }
    result.into()
}

#[proc_macro_derive(Challenge, attributes(challenge))]
pub fn derive_challenge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    // parse the list of types for which to impl
    let types = match get_types("Challenge", NAME_CHALLENGE, &ast) {
        Ok(types) => types,
        Err(err) => return err.to_compile_error().into(),
    };

    // join all the implementations
    let mut result = proc_macro::TokenStream::new();
    for tt in types.0.iter() {
        result.extend(impl_challenge(tt, &ast).into_iter())
    }
    result.into()
}
