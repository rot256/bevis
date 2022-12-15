use proc_macro;

use proc_macro2::TokenStream;

use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

use quote::{quote, quote_spanned};

use proc_macro2::{Ident, Span};

// Add a bound `T: Msg` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(fsffs::Msg));
        }
    }
    generics
}

fn check_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => {
            let children = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! { is_msg(&self.#name); }
            });
            quote! {
                #(#children)*
            }
        }
        Fields::Unnamed(ref fields) => {
            // Expands to an expression like
            //
            //     0 + self.0.heap_size() + self.1.heap_size() + self.2.heap_size()
            let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                let index = Index::from(i);
                quote! { is_msg(&self.#index); }
            });
            quote! {
                #(#recurse)*
            }
        }
        Fields::Unit => {
            quote!{}
        }
    }
}

fn type_check_children(data: &Data) -> TokenStream {
    let checks = match *data {
        Data::Struct(ref data) => check_fields(&data.fields),
        Data::Enum(ref data) => {
        
            let arms = data.variants.iter().map(|variant| {
                let ident = &variant.ident;

                // not sound, need sep for every variant, consider the trivial case where it is a bool.
                match variant.fields {
                    Fields::Named(ref fields) => {
                        let names = fields.named.iter().map(|f| {
                            &f.ident
                        });

                        let checks = fields.named.iter().map(|f| {
                            let name = &f.ident;
                            quote!{ is_msg(#name); }
                        });

                        quote!{
                            Self::#ident{#(#names)*} => {
                                #(#checks)*
                            },
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let names = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                            Ident::new(&format!("n{}", i), Span::call_site())
                        });

                        let checks = names.clone().map(|name| {
                            quote!{ is_msg(#name); }
                        });
                        
                        quote!{
                            Self::#ident(#(#names)*) => {
                                #(#checks)*
                            },
                        }
                    }
                    Fields::Unit => {
                        quote!{}
                    }
                }
            });
            quote! {  
                match self { 
                    #(#arms)* 
                }
            }
        }
        Data::Union(ref data) => {
            check_fields(&Fields::Named(data.fields.clone()))
        }
    };

    // add checker
    quote! {
        fn is_msg<M: Msg>(t: &M) {}
        #checks
    }
}

#[proc_macro_derive(Msg)]
pub fn derive_msg(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // ensure that the children are also Msg
    let type_checks = type_check_children(&input.data);

    // implement Msg for the parent
    let name = input.ident;
    let expanded = quote! {
        impl #impl_generics fsffs::Msg for #name #ty_generics #where_clause {}
        impl #impl_generics fsffs::private::Seal for #name #ty_generics #where_clause {
            fn check(&self) {
                #type_checks
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// if it contains only msg, then it is msg



#[cfg(test)]
mod tests {

}
