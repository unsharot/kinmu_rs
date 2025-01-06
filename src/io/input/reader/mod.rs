//! toml形式のconfigをそのまま取り込むモジュール

use types::{RawAnnealingConfig, RawMainConfig, RawScheduleConfig};

pub mod types;

use std::fs;
use toml;

pub fn read_main_config(path: &str) -> Result<RawMainConfig, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: RawMainConfig = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}

pub fn read_schedule_config(path: &str) -> Result<RawScheduleConfig, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: RawScheduleConfig = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}

pub fn read_annealing_config(path: &str) -> Result<RawAnnealingConfig, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: RawAnnealingConfig = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}
