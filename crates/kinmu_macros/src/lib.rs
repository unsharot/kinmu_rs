//! modelに依存するマクロを提供

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AngleBracketedGenericArguments, DeriveInput};

fn get_generic_attribute(input: &DeriveInput) -> syn::Result<AngleBracketedGenericArguments> {
    for a in &input.attrs {
        if a.path().is_ident("score_prop_trait") {
            return match a.parse_args::<AngleBracketedGenericArguments>() {
                Ok(v) => {
                    if v.args.len() != 3 {
                        Err(syn::Error::new_spanned(
                            a.path().get_ident(),
                            format!("expected 3 fields, found {}", v.args.len()),
                        ))
                    } else {
                        Ok(v)
                    }
                }
                Err(e) => Err(syn::Error::new_spanned(a.path().get_ident(), e)),
            };
        }
    }
    Err(syn::Error::new_spanned(
        &input.ident,
        "must have score_prop_trait attribute",
    ))
}

/// ScorePropをenumで拡張するためのderiveマクロ
#[proc_macro_derive(ScorePropTrait, attributes(score_prop_trait))]
pub fn derive_score_prop(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let data_enum = match &input.data {
        syn::Data::Enum(v) => v,
        _ => {
            return syn::Error::new_spanned(&input.ident, "must be enum type")
                .to_compile_error()
                .into();
        }
    };

    let mut variants = Vec::new();
    for v in &data_enum.variants {
        variants.push(v.ident.clone());
    }

    let generics = match get_generic_attribute(&input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let trait_path: syn::Path = parse_quote!(kinmu_model::ScorePropTrait);
    let ty = input.ident;
    let (impl_g, ty_g, where_clause) = input.generics.split_for_impl();

    let shift = generics.args.get(0).unwrap();
    let shift_state = generics.args.get(1).unwrap();
    let day_state = generics.args.get(2).unwrap();

    let gen = quote! {
        #[automatically_derived]
        impl #impl_g #trait_path #generics for #ty #ty_g #where_clause {
            fn eval_mut(&mut self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> kinmu_model::Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_mut(x, staff_config, day_config, schedule),)*
                }
            }

            fn eval_immut(&self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_immut(x, staff_config, day_config, schedule),)*
                }
            }
        }
    };

    gen.into()
}
