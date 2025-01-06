use crate::kinmu_lib::types::ScoreProp;

/// 焼きなましの段階ごとの設定
#[derive(Clone)]
pub struct AnnealingConfig {
    pub step: u32,                   // 焼きなましのステップ数
    pub seed: u64,                   // 焼きなましのupdate関数の乱数のシード
    pub score_props: Vec<ScoreProp>, // 焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32,
}
