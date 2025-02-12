//! annealing_configを定義

use super::super::ScoreProp;

/// 焼きなましの段階ごとの設定
#[derive(Clone, Debug, Default)]
pub struct AnnealingConfig {
    pub step: u32,                   // 焼きなましのステップ数
    pub seed: Option<u64>,           // 焼きなましのupdate関数の乱数のシード
    pub score_props: Vec<ScoreProp>, // 焼きなましのためのスコア
    pub update_func: String,
    pub max_temp: f32,
    pub min_temp: f32,
}
