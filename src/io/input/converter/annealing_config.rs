//! annealing_configを読み込むモジュール

use super::super::reader::types::RawAnnealingConfig;
use crate::kinmu_lib::types::{AnnealingConfig, ScoreProp};

use super::util::parser::*;

/// 焼きなましの段階ごとの設定を読み込む
pub fn convert_annealing_config(config: RawAnnealingConfig) -> anyhow::Result<AnnealingConfig> {
    let ac = AnnealingConfig {
        step: config.step_count,
        seed: config.seed,
        score_props: config
            .score_functions
            .iter()
            .map(|sf| {
                sf.scores
                    .iter()
                    .map(|s| ScoreProp::from_config(s))
                    .collect::<anyhow::Result<Vec<ScoreProp>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<ScoreProp>>>>()?
            .concat(),
        update_func: config.update_function,
        max_temp: config.temp.max,
        min_temp: config.temp.min,
    };

    Ok(ac)
}
