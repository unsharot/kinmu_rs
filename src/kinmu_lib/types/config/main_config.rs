//! main_configを定義

pub type FilePath = String;

pub struct MainConfig {
    pub schedule_config_paths: Vec<FilePath>,
    pub thread_count: Option<u32>,
}
