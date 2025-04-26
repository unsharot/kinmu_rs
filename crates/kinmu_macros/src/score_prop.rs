//! ScorePropをderiveするためのマクロの詳細実装

use quote::quote;
use syn::{parse_quote, AngleBracketedGenericArguments, DeriveInput};

/// ScorePropのderiveマクロの詳細実装
pub fn derive_score_prop(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // enumか判定
    let data_enum = match &input.data {
        syn::Data::Enum(v) => Ok(v),
        _ => Err(syn::Error::new_spanned(&input.ident, "must be enum type")),
    }?;

    // enumの列挙子を取得
    let mut variants = Vec::new();
    for v in &data_enum.variants {
        variants.push(v.ident.clone());
    }

    // ScorePropのジェネリクスをattributeから取得
    let generics = get_generic_attribute(&input)?;

    // ジェネリクスの型引数を個別に取得
    let shift = generics.args.get(0).unwrap();
    let shift_state = generics.args.get(1).unwrap();
    let day_state = generics.args.get(2).unwrap();

    // ScorePropのパス
    let trait_path: syn::Path = parse_quote!(kinmu_model::ScoreProp);

    // ScorePropのderive先の識別子
    let ty = input.ident;

    // 埋め込み対象のジェネリクスの要素を分割
    let (impl_g, ty_g, where_clause) = input.generics.split_for_impl();

    // implのコードを生成
    let gen = quote! {
        #[automatically_derived]
        impl #impl_g #trait_path #generics for #ty #ty_g #where_clause {
            fn eval_mut(&mut self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> kinmu_model::Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_mut(x, staff_config, day_config, schedule),)*
                }
            }

            fn eval_immut(&self, staff_config: &kinmu_model::StaffConfig, day_config: &kinmu_model::DayConfig<#shift, #shift_state, #day_state>, schedule: &kinmu_model::Schedule<#shift>) -> kinmu_model::Score {
                match self {
                    #(Self::#variants(x) => #trait_path::eval_immut(x, staff_config, day_config, schedule),)*
                }
            }
        }
    };

    // proc_macro2::TokenStreamをResultでラップしてリターン
    Ok(gen)
}

/// score_propのattributeを取得してgenericsを返す
fn get_generic_attribute(input: &DeriveInput) -> syn::Result<AngleBracketedGenericArguments> {
    for a in &input.attrs {
        if a.path().is_ident("score_prop") {
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
        "must have score_prop attribute",
    ))
}
