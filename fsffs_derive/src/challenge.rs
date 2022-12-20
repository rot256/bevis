use super::*;

pub fn impl_challenge(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(fsffs::Challenge));
            }
        }
        generics
    }

    //
    let input = parse_macro_input!(input as DeriveInput);
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    fn body(data: Data) -> TokenStream {
        let fn_name = quote! { fsffs::Absorb::absorb };
        match data {
            Data::Union(_) => 
                syn::Error::new(
                    fn_name.span(),
                    "cannot sample union",
                ).to_compile_error(),

            Data::Enum(_) => 
                syn::Error::new(
                    fn_name.span(),
                    "cannot sample enum (yet)",
                ).to_compile_error(),

            Data::Struct(ref data) => {
                match data.fields {
                    Fields::Named(ref fields) => {
                        let names = fields.named.iter().map(|f| {
                            &f.ident
                        });

                        let children = names.clone().map(|name| {
                            quote! { let #name = fsffs::Challenge::sample(__sampler); }
                        });
                     
                        quote! {
                            #(#children)*
                            Self { #(#names,)* }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let names = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _f)| format_ident!("n{}", i));

                        let children = names.clone().map(|name| {
                                quote! { let #name = fsffs::Challenge::sample(__sampler); }
                        });

                        quote! {
                            #(#children)*
                            Self ( #(#names,)* )
                        }
                    }
                    Fields::Unit => {
                        quote! {} // nothing to do
                    }
                }
            }
        }
    }

    // compute body of impl function
    let sampler = body(input.data);

    // implement Msg for the parent
    let name = input.ident;
    let expanded = quote! {
        impl #impl_generics fsffs::Challenge for #name #ty_generics #where_clause {
            fn sample<S: fsffs::Sponge>(__sampler: &mut S) -> Self {
                #sampler
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}