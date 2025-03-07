use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AngleBracketedGenericArguments, DeriveInput};

fn get_generic_attribute(input: &DeriveInput) -> syn::Result<AngleBracketedGenericArguments> {
    for a in &input.attrs {
        if a.path().is_ident("score_prop_trait") {
            match a.parse_args() {
                Ok(v) => return Ok(v),
                Err(e) => {
                    return Err(syn::Error::new_spanned(a.path().get_ident(), e));
                }
            };
        }
    }
    Err(syn::Error::new_spanned(
        &input.ident,
        "Must have score_prop_tait attribute",
    ))
}

#[proc_macro_derive(ScorePropTrait, attributes(score_prop_trait))]
pub fn derive_score_prop(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    // let mut generics: AngleBracketedGenericArguments;
    // for a in input.attrs {
    //     if a.path().is_ident("score_prop_trait") {
    //         generics = match a.parse_args() {
    //             Ok(v) => v,
    //             Err(e) => {
    //                 return syn::Error::new_spanned(a.path().get_ident(), e).to_compile_error().into();
    //             }
    //         };
    //     }
    // }

    let data_enum = match &input.data {
        syn::Data::Enum(v) => v,
        _ => {
            return syn::Error::new_spanned(&input.ident, "Must be enum type")
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

    // dbg!(&input.generics);

    // let sp = generics.args.get(0).unwrap();
    let shift = generics.args.get(0).unwrap();
    let shift_state = generics.args.get(1).unwrap();
    let day_state = generics.args.get(2).unwrap();

    // let sp = "ScoreProp";
    // let shift = "Shift";
    // let shift_state = "ShiftState";
    // let day_state = "DayState";

    let gen = quote! {
        #[automatically_derived]
        impl #impl_g #trait_path #generics for #ty #ty_g #where_clause {
            fn eval_mut(&mut self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> kinmu_model::Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_mut(self, staff_config, day_config, schedule),)*
                }
            }

            fn eval_immut(&self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_immut(self, staff_config, day_config, schedule),)*
                }
            }
        }
    };

    // dbg!(&gen);

    gen.into()
}
