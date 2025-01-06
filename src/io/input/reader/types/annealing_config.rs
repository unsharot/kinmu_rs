//! annealing_configを読み込むための構造体

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingConfig {
    pub step_count: u32,
    pub seed: u64,
    pub score_functions: Vec<String>,
    pub update_function: String,
    pub temp: RawTempKey,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawTempKey {
    pub max: f32,
    pub min: f32,
}
