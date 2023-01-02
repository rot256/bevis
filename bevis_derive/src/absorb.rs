use super::*;

fn absorb_fields(fn_name: TokenStream, fields: &Fields) -> TokenStream {
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

pub fn impl_absorb(input: &syn::DeriveInput) -> proc_macro::TokenStream {
    fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(::bevis::Absorb));
            }
        }
        generics
    }

    fn body(data: &Data) -> TokenStream {
        let fn_name = quote! { ::bevis::Absorb::absorb };
        match data {
            Data::Union(_) => syn::Error::new(fn_name.span(), "absorb not implemented for union.")
                .to_compile_error(),

            Data::Struct(ref data) => absorb_fields(fn_name, &data.fields),

            Data::Enum(ref data) => {
                // no need to absorb empty enum
                if data.variants.len() == 0 {
                    return quote! {};
                }

                // there is space: I can't forsee needing more than 2^31
                assert!(data.variants.len() < (1 << 31));

                // handle each variant
                let chls = data.variants.iter().map(|variant| {
                    let ident = &variant.ident;
                    match variant.fields {
                        Fields::Named(ref fields) => {
                            let names = fields.named.iter().map(|f| &f.ident);

                            let checks = fields.named.iter().map(|f| {
                                let name = &f.ident;
                                quote! { #fn_name(#name, h); }
                            });

                            quote! {
                                Self::#ident{#(#names,)*} => {
                                    #(#checks)*;
                                    ()
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
                                quote! { #fn_name(#name, h); }
                            });

                            quote! {
                                Self::#ident(#(#names,)*) => {
                                    #(#checks)*;
                                    ()
                                },
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                Self::#ident => { () },
                            }
                        }
                    }
                });

                let encs = data.variants.iter().enumerate().map(|(i, variant)| {
                    let ident = &variant.ident;
                    match variant.fields {
                        Fields::Named(_) => {
                            quote! {
                                Self::#ident{ .. } => { #i }
                            }
                        }
                        Fields::Unnamed(_) => {
                            quote! {
                                Self::#ident( .. ) => { #i }
                            }
                        }
                        Fields::Unit => {
                            quote! {
                                Self::#ident => #i,
                            }
                        }
                    }
                });

                // choose smallest possible integer type
                let varid = {
                    let n = data.variants.len() as u64;
                    if n <= u8::MAX.into() {
                        quote! { let varid: u8 = match self { #(#encs)* } as u8; }
                    } else if n <= u16::MAX.into() {
                        quote! { let varid: u16 = match self { #(#encs)* } as u16; }
                    } else if n <= u32::MAX.into() {
                        quote! { let varid: u32 = match self { #(#encs)* } as u32; }
                    } else {
                        quote! { let varid: u64 = match self { #(#encs)* } as u64; }
                    }
                };

                quote! {
                    // encode variant
                    #varid

                    // absorb variant encoding
                    #fn_name(&varid, h);

                    // encode child
                    match self {
                        #(#chls)*
                    }
                }
            }
        }
    }

    //
    let generics = add_trait_bounds(input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // compute body of impl function
    let checks = body(&input.data);

    // implement Msg for the parent
    let name = input.ident.clone();
    let expanded = quote! {
        impl #impl_generics ::bevis::Absorb for #name #ty_generics #where_clause {
            fn absorb<H: ::bevis::Hasher>(&self, h: &mut H) {
                #checks
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
