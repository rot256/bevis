use super::*;

pub fn impl_transcript(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    fn add_trait_bounds(mut generics: Generics) -> Generics {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = *param {
                type_param.bounds.push(parse_quote!(fsffs::Tx));
            }
        }
        generics
    }

    //
    let input = parse_macro_input!(input as DeriveInput);
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // ensure that the children are also Msg
    let fn_name = quote! { fsffs::Tx::read };

    // generate body of impl function
    let checks = match input.data {
        Data::Union(ref data) => check_fields(fn_name, &Fields::Named(data.fields.clone())),
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
        impl #impl_generics fsffs::Tx for #name #ty_generics #where_clause {
            fn read<A: fsffs::Arthur>(&self, ts: &mut A) {
                #checks
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
