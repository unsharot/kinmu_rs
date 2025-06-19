//! annealing_configを読み込むための構造体

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingConfig {
    pub step_count: u32,
    pub seed: Option<u64>,
    pub score_functions: Vec<RawAnnealingScoreFunction>,
    pub update_function: String,
    pub temperature: RawTemperatureKey,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingScoreFunction {
    pub scores: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTemperatureKey {
    pub begin: f32,
    pub end: f32,
}
