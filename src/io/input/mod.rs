//! toml形式のconfigを読み込むためのモジュール
//! パスを受け取り、configを返す

use crate::kinmu_lib::types::{AnnealingConfig, FilePath, FillConfig, MainConfig, ScheduleConfig};

mod checker;
mod converter;
mod reader;

pub fn load_main_config(path: &str) -> Result<MainConfig, String> {
    let raw = reader::read_main_config(path)?;
    let converted = converter::convert_main_config(raw)?;
    Ok(converted)
}

pub fn load_schedule_config(
    path: &str,
) -> Result<(ScheduleConfig, Vec<FilePath>, FillConfig), String> {
    let raw = reader::read_schedule_config(path)?;
    let converted = converter::convert_schedule_config(raw)?;
    checker::check_schedule_config(&converted.0)?;
    Ok(converted)
}

pub fn load_annealing_config(
    path: &str,
    schedule_config: &ScheduleConfig,
) -> Result<AnnealingConfig, String> {
    let raw = reader::read_annealing_config(path)?;
    let converted = converter::convert_annealing_config(raw)?;
    checker::check_annealing_config(&converted, schedule_config)?;
    Ok(converted)
}
