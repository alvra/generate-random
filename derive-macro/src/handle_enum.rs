use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use syn::{DataEnum, Variant};
use super::generate_fields;

fn variant_weight(variant: &Variant) -> Literal {
    for attr in variant.attrs.iter() {
        if attr.path.is_ident("weight") {
            return attr.parse_args::<Literal>().expect("expected literal for `#[weight(...)]`")
        }
    }
    Literal::u64_suffixed(1)
}

pub fn generate(name: &Ident, ty: DataEnum) -> TokenStream {
    let mut variant_weights = ty.variants.into_iter()
        .map(|variant| (variant_weight(&variant), variant));

    let mut arms = TokenStream::new();
    let mut total_weight = quote! { 0 };
    if let Some((weight, variant)) = variant_weights.next() {
        let variant_name = variant.ident;
        let fields = generate_fields(variant.fields);
        arms.extend(quote! {
            let end = #weight;
            if 0 <= value && value < end {
                return Self::#variant_name #fields
            }
        });
        total_weight = quote! { #weight };
        for (weight, variant) in variant_weights {
            let variant_name = variant.ident;
            let fields = generate_fields(variant.fields);
            arms.extend(quote! {
                let start = end;
                let end = start + #weight;
                if start <= value && value < end {
                    return Self::#variant_name #fields
                }
            });
            total_weight = quote! { #total_weight + #weight };
        }
    }

    quote! {
        impl generate_random::GenerateRandom for #name {
            fn generate_random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
                let total_weight = #total_weight;
                let value = rng.gen_range(0..total_weight);
                #arms
                unreachable!()
            }
        }
    }
}
