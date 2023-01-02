use super::*;

pub(crate) fn impl_challenge(tt: &Ident, input: &syn::DeriveInput) -> proc_macro::TokenStream {
    fn add_trait_bounds(tt: &Ident, mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param
                    .bounds
                    .push(parse_quote!(::bevis::Challenge<#tt>));
            }
        }
        generics.clone()
    }

    //
    let generics = add_trait_bounds(tt, input.generics.clone());
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    fn body(tt: &Ident, data: &Data) -> TokenStream {
        let fn_name = quote! { ::bevis::Absorb::absorb };
        match data {
            Data::Union(_) =>
                syn::Error::new(
                    fn_name.span(),
                    "cannot sample union: variants might have different samplers.",
                ).to_compile_error(),

            Data::Enum(_) =>
                // uniform distribution might not be ideal, 
                // e.g. KKW18 uses a skewed distribution.
                syn::Error::new(
                    fn_name.span(),
                    "cannot sample enum: it remains to decide what/how the dist. over variants should be chosen.",
                ).to_compile_error(),

            Data::Struct(ref data) => {
                match data.fields {
                    Fields::Named(ref fields) => {
                        let children = fields.named.iter().map(|field| {
                            let name = &field.ident;
                            quote! {
                                #name: ::bevis::Challenge::<#tt>::sample(s)
                            }
                        });

                        quote! {
                            Self { #(#children,)* }
                        }
                    }
                    Fields::Unnamed(ref fields) => {
                        let children = fields.unnamed.iter().map(|_field| {
                            quote! {
                                ::bevis::Challenge::<#tt>::sample(s)
                            }
                        });

                        quote! {
                            Self ( #(#children,)* )
                        }
                    }
                    Fields::Unit => {
                        // nothing to do: 
                        // the sample space contains a single value
                        quote! {
                            Self
                        }
                    }
                }
            }
        }
    }

    // compute body of impl function
    let sampler = body(tt, &input.data);

    // implement Msg for the parent
    let name = input.ident.clone();
    let expanded = quote! {
        impl #impl_generics ::bevis::Challenge<#tt> for #name #ty_generics #where_clause {
            fn sample<S: ::bevis::Sampler::<#tt>>(s: &mut S) -> Self {
                #sampler
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
