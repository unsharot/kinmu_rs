//! annealing_configを読み込むモジュール

use super::{super::reader::types::RawAnnealingConfig, FromConfig};
use ::kinmu_model::AnnealingConfig;

/// 焼きなましの段階ごとの設定を読み込む
pub fn convert_annealing_config<SP: FromConfig + Clone>(
    config: RawAnnealingConfig,
) -> anyhow::Result<AnnealingConfig<SP>> {
    let ac = AnnealingConfig {
        step: config.step_count,
        seed: config.seed,
        score_props: config
            .score_functions
            .iter()
            .map(|sf| {
                sf.scores
                    .iter()
                    .map(|s| SP::from_config(s))
                    .collect::<anyhow::Result<Vec<SP>>>()
            })
            .collect::<anyhow::Result<Vec<Vec<SP>>>>()?
            .concat(),
        update_func: config.update_function,
        max_temp: config.temp.max,
        min_temp: config.temp.min,
    };

    Ok(ac)
}
