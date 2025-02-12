//! main_configを読み込むための構造体

#[derive(Debug, serde::Deserialize)]
pub struct RawMainConfig {
    pub schedule_config_paths: Vec<String>,
    pub thread_count: Option<u32>,
}
