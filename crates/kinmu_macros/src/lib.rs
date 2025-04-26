//! modelに依存するマクロを提供

mod score_prop;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// ScorePropをenumで拡張するためのderiveマクロ
///
/// 使用例
/// ```
/// use kinmu_macros::ScoreProp;
/// use kinmu_model::{ScoreProp, Score, StaffConfig, DayConfig, Schedule};
///
/// #[derive(Default)]
/// struct S;
/// #[derive(Default)]
/// struct SS;
/// #[derive(Default)]
/// struct DS;
///
/// struct AScoreProp;
/// struct BScoreProp;
///
/// impl ScoreProp<S, SS, DS> for AScoreProp {
///     fn eval_mut(
///         &mut self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         0.0
///     }
///     fn eval_immut(
///         &self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         1.0
///     }
/// }
/// impl ScoreProp<S, SS, DS> for BScoreProp {
///     fn eval_mut(
///         &mut self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         0.1
///     }
///     fn eval_immut(
///         &self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         1.1
///     }
/// }
///
/// #[derive(ScoreProp)]
/// #[score_prop(<S, SS, DS>)]
/// enum SomeScoreProp {
///     PatternA(AScoreProp),
///     PatternB(BScoreProp),
/// }
///
/// let mut ssp1 = SomeScoreProp::PatternA(AScoreProp);
/// let mut ssp2 = SomeScoreProp::PatternB(BScoreProp);
///
/// let staff_config = Default::default();
/// let day_config = Default::default();
/// let schedule = Default::default();
///
/// assert_eq!(ssp1.eval_mut(&staff_config, &day_config, &schedule), 0.0);
/// assert_eq!(ssp1.eval_immut(&staff_config, &day_config, &schedule), 1.0);
/// assert_eq!(ssp2.eval_mut(&staff_config, &day_config, &schedule), 0.1);
/// assert_eq!(ssp2.eval_immut(&staff_config, &day_config, &schedule), 1.1);
///
/// ```
///
/// このderiveマクロは以下のコードと同じ意味のコードを生成します。
///
/// ```ignore
/// impl ScoreProp<S, SS, DS> for SomeScoreProp {
///     fn eval_mut(
///         &mut self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         match self {
///             Self::APattern(x) => {
///                 x.eval_mut(staff_config, day_config, schedule)
///             }
///             Self::BPattern(x) => {
///                 x.eval_mut(staff_config, day_config, schedule)
///             }
///         }
///     }
///     fn eval_immut(
///         &self,
///         staff_config: &StaffConfig,
///         day_config: &DayConfig<S, SS, DS>,
///         schedule: &Schedule<S>,
///     ) -> Score {
///         match self {
///             Self::APattern(x) => {
///                 x.eval_immut(staff_config, day_config, schedule)
///             }
///             Self::BPattern(x) => {
///                 x.eval_immut(staff_config, day_config, schedule)
///             }
///         }
///     }
/// }
/// ```
#[proc_macro_derive(ScoreProp, attributes(score_prop))]
pub fn derive_score_prop(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    score_prop::derive_score_prop(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
