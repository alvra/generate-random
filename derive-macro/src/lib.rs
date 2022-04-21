//! This crate provide the [`GenerateRandom`] derive macro
//! that implements the trait of the same name from the `generate-random` crate.
//! Refer to the documentation of that crate for more information.

use syn::{DeriveInput, Data, Fields};

mod handle_struct;
mod handle_enum;

#[proc_macro_derive(GenerateRandom, attributes(weight))]
pub fn derive_generate_random(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    match input.data {
        Data::Struct(ty) => handle_struct::generate(&input.ident, ty),
        Data::Enum(ty) => handle_enum::generate(&input.ident, ty),
        Data::Union(_) => panic!("Unions are not supported"),
    }.into()
}

fn generate_fields(fields: Fields) -> proc_macro2::TokenStream {
    use quote::quote;
    match fields {
        Fields::Named(fields) => {
            let fields = fields.named.into_iter()
                .map(|field| {
                    let field = field.ident.unwrap();
                    quote! {
                        #field: generate_random::GenerateRandom::generate_random(rng),
                    }
                })
                .collect::<proc_macro2::TokenStream>();
            quote! { { #fields } }
        }
        Fields::Unnamed(fields) => {
            let fields = fields.unnamed.into_iter()
                .map(|_field| {
                    quote! {
                        generate_random::GenerateRandom::generate_random(rng),
                    }
                })
                .collect::<proc_macro2::TokenStream>();
            quote! { ( #fields ) }
        }
        Fields::Unit => quote! {},
    }
}
