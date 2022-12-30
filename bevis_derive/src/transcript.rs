use super::*;

// helper function (used by Absorb and Transcript)
fn check_fields(fn_name: TokenStream, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(ref fields) => {
            let children = fields.named.iter().map(|f| {
                let name = &f.ident;
                quote! { #fn_name(&self.#name); }
            });
            quote! {
                #(#children)*
            }
        }
        Fields::Unnamed(ref fields) => {
            let children = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                let index = Index::from(i);
                quote! { #fn_name(&self.#index); }
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

pub fn impl_transcript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(bevis::Tx));
            }
        }
        generics
    }

    //
    let input = parse_macro_input!(input as DeriveInput);
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // ensure that the children are also Msg
    let fn_name = quote! { bevis::Tx::read };

    // generate body of impl function
    let checks = match input.data {
        Data::Union(_) => syn::Error::new(fn_name.span(), "transcript not implemented for union")
            .to_compile_error(),

        Data::Struct(ref data) => check_fields(fn_name, &data.fields),

        Data::Enum(_) => syn::Error::new(
            fn_name.span(),
            "enums cannot implement transcript (derive absorb instead)",
        )
        .to_compile_error(),
    };

    // implement transcript
    let name = input.ident;
    let expanded = quote! {
        impl #impl_generics bevis::Tx for #name #ty_generics #where_clause {
            #[inline(always)]
            fn read(&self) {
                #checks
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
