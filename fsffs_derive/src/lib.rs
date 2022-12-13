use proc_macro;

use proc_macro2::TokenStream;

use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

use quote::{quote, quote_spanned};


// Add a bound `T: Msg` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    println!("param: {}", generics.params.len());
    for param in &mut generics.params {
        
        if let GenericParam::Type(ref mut type_param) = *param {
            println!("a");
            type_param.bounds.push(parse_quote!(fsffs::Msg));
        }
    }
    generics
}

fn heap_size_sum(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    println!("fields: {}", fields.named.len());
                    // Expands to an expression like
                    //
                    //     0 + self.x.heap_size() + self.y.heap_size() + self.z.heap_size()
                    //
                    // but using fully qualified function call syntax.
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `heap_size_of_children`
                    // call. This way if one of the field types does not
                    // implement `HeapSize` then the compiler's error message
                    // underlines which field it is. An example is shown in the
                    // readme of the parent directory.
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote! { is_msg(&self.#name); }
                    });

                    quote! {
                        {
                            fn is_msg<M: Msg>(t: &M) {}
                            #(#recurse)*
                        }
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
                        {
                            fn is_msg<M: Msg>(t: &M) {}
                            #(#recurse)*
                        }
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(Msg)]
pub fn derive_msg(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    println!("field");
    let input = parse_macro_input!(input as DeriveInput);

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;


    let sum = heap_size_sum(&input.data);

    let expanded = quote! {

        impl #impl_generics fsffs::Msg for #name #ty_generics #where_clause {}

        impl #impl_generics fsffs::private::Seal for #name #ty_generics #where_clause {
            fn check(&self) {
                #sum
            }
        }
    };


    proc_macro::TokenStream::from(expanded)
}

// if it contains only msg, then it is msg



#[cfg(test)]
mod tests {

}
