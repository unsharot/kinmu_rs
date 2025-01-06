use crate::kinmu_lib::types::{AnnealingConfig, ScoreProp};

use super::super::super::reader;
use super::super::parser::*;

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> Result<AnnealingConfig, String> {
    let config = reader::read_annealing_config(path)?;

    let ac = AnnealingConfig {
        step: config.step_count,
        seed: config.seed,
        score_props: config
            .score_functions
            .iter()
            .map(|s| ScoreProp::from_config(s))
            .collect::<Result<Vec<ScoreProp>, String>>()?,
        update_func: config.update_function,
        max_temp: config.temp.max,
        min_temp: config.temp.min,
    };

    Ok(ac)
}
