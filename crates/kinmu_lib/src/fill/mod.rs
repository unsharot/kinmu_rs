//! 初めに勤務表を埋める関数のモジュール

/*
fill1はUをランダムな枠に
fill2はIとKの数合わせてうまいこと埋める
*/

use super::{DayState, Schedule, ScheduleConfig, Shift, ShiftState, StdScoreProp};

use kinmu_generator_with_annealing::Fill;

use rand::Rng;

mod fill_iak_safe;
mod fill_noh;
mod no_fill;

use fill_iak_safe::fill_iak_safe;
use fill_noh::fill_noh;
use no_fill::no_fill;

/// 生成器で用いるFill関数のための型
/// GeneratorWithAnnealingのFillを実装
#[derive(Debug, Clone)]
pub struct StdFill;

/// Fillの実装
impl Fill<StdScoreProp, Shift, ShiftState, DayState> for StdFill {
    fn run<R: Rng>(
        &self,
        name: &str,
        schedule_config: &ScheduleConfig,
        mut rng: &mut R,
    ) -> anyhow::Result<Schedule> {
        match name {
            "no_fill" => Ok(no_fill(schedule_config, &mut rng)),
            "fill_noh" => Ok(fill_noh(schedule_config, &mut rng)),
            "fill_iak_safe" => fill_iak_safe(schedule_config, &mut rng),
            _ => Err(anyhow::anyhow!("Failed to parse fill function {}", name)),
        }
    }
}
