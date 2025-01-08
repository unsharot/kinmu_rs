//! main_configを定義

use super::super::FilePath;

pub struct MainConfig {
    pub schedule_config_paths: Vec<FilePath>,
    pub thread_count: Option<u32>,
}
