//! modelに依存するマクロを提供

mod score_prop;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// ScorePropをenumで拡張するためのderiveマクロ
#[proc_macro_derive(ScoreProp, attributes(score_prop))]
pub fn derive_score_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    score_prop::derive_score_prop(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
