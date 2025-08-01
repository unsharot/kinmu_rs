//! toml形式のconfigをそのまま取り込むモジュール

use types::{RawAnnealingConfig, RawMainConfig, RawScheduleConfig};

pub(super) mod types;

use std::fs;
use std::path::Path;

/// main_configを読み込む
pub(super) fn read_main_config<P: AsRef<Path> + ?Sized>(path: &P) -> anyhow::Result<RawMainConfig> {
    let contents = fs::read_to_string(path).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let config: RawMainConfig =
        toml::from_str(&contents).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(config)
}

/// schedule_configを読み込む
pub(super) fn read_schedule_config<P: AsRef<Path> + ?Sized>(
    path: &P,
) -> anyhow::Result<RawScheduleConfig> {
    let contents = fs::read_to_string(path).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let config: RawScheduleConfig =
        toml::from_str(&contents).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(config)
}

/// annealing_configを読み込む
pub(super) fn read_annealing_config<P: AsRef<Path> + ?Sized>(
    path: &P,
) -> anyhow::Result<RawAnnealingConfig> {
    let contents = fs::read_to_string(path).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let config: RawAnnealingConfig =
        toml::from_str(&contents).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(config)
}
