use super::*;

pub fn impl_absorb(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(fsffs::Absorb));
            }
        }
        generics
    }

    fn body(data: Data) -> TokenStream {
        let fn_name = quote! { fsffs::Absorb::absorb };
        match data {
            Data::Union(ref data) => check_fields(fn_name, &Fields::Named(data.fields.clone())),

            Data::Struct(ref data) => check_fields(fn_name, &data.fields),

            Data::Enum(ref data) => {
                // no need to encode empty enum
                if data.variants.len() == 0 {
                    return quote! {};
                }

                // there is space
                assert!(data.variants.len() < (1 << 31));

                // handle each variant
                let arms = data.variants.iter().enumerate().map(|(i, variant)| {
                    let varid: TokenStream = format!("&{}u32", i).parse().unwrap();
                    let ident = &variant.ident;

                    // not sound, need sep for every variant, consider the trivial case where it is a bool.
                    match variant.fields {
                        Fields::Named(ref fields) => {
                            let names = fields.named.iter().map(|f| {
                                println!("{:?}", &f.ident);
                                &f.ident
                            });

                            let checks = fields.named.iter().map(|f| {
                                let name = &f.ident;
                                quote! { #fn_name(#name, ts); }
                            });

                            quote! {
                                Self::#ident{#(#names,)*} => {
                                    #fn_name(#varid, ts);
                                    #(#checks)*
                                },
                            }
                        }
                        Fields::Unnamed(ref fields) => {
                            let names = fields
                                .unnamed
                                .iter()
                                .enumerate()
                                .map(|(i, _f)| format_ident!("n{}", i));

                            let checks = names.clone().map(|name| {
                                quote! { #fn_name(#name, ts); }
                            });

                            quote! {
                                Self::#ident(#(#names,)*) => {
                                    #fn_name(#varid, ts);
                                    #(#checks)*
                                },
                            }
                        }
                        Fields::Unit => {
                            quote! {}
                        }
                    }
                });

                // match over variants
                quote! {
                    match self {
                        #(#arms)*
                    }
                }
            }
        }
    }

    //
    let input = parse_macro_input!(input as DeriveInput);
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // compute body of impl function
    let checks = body(input.data);

    // implement Msg for the parent
    let name = input.ident;
    let expanded = quote! {
        impl #impl_generics fsffs::Absorb for #name #ty_generics #where_clause {
            fn absorb<A: fsffs::Arthur>(&self, ts: &mut A) {
                #checks
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
