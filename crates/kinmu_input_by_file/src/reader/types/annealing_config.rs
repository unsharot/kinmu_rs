//! annealing_configを読み込むための構造体

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingConfig {
    pub step_count: u32,
    pub seed: Option<u64>,
    pub score_functions: Vec<RawAnnealingScoreFunction>,
    pub update_function: String,
    pub temp: RawTempKey,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingScoreFunction {
    pub scores: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTempKey {
    pub max: f32,
    pub min: f32,
}
