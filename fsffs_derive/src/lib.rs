use proc_macro;
use proc_macro2::TokenStream;

use quote::{format_ident, quote};

use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

mod absorb;
mod transcript;
mod challenge;

// derive of Absorb
use absorb::impl_absorb;

// derive of Transcript
use transcript::impl_transcript;

// derieve of Challenge
use challenge::impl_challenge;

// helper function (used by Absorb and Transcript)
fn hash_fields(fn_name: TokenStream, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => {
            let children = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! { #fn_name(&self.#name, h); }
            });
            quote! {
                #(#children)*
            }
        }
        Fields::Unnamed(ref fields) => {
            let children = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                let index = Index::from(i);
                quote! { #fn_name(&self.#index, h); }
            });
            quote! {
                #(#children)*
            }
        }
        Fields::Unit => {
            quote! {} // nothing to do: no fields to hash
        }
    }
}

#[proc_macro_derive(Tx)]
pub fn derive_transcript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_transcript(input)
}

#[proc_macro_derive(Absorb)]
pub fn derive_absorb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_absorb(input)
}

#[proc_macro_derive(Challenge)]
pub fn derive_challenge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_challenge(input)
}