//! 焼きなましで使う更新関数のモジュール

use super::{DayState, Schedule, ScheduleConfig, Shift, ShiftState, StdScoreProp};

use kinmu_generator_with_annealing::Update;

use rand::Rng;

mod iak_safe;
mod iaknoh;
mod iaknoh_repeat;
mod noh_repeat;
mod range;
mod range_or_iaknoh_repeat;
mod range_or_noh_repeat;

use iak_safe::update_iak_safe;
use iaknoh::update_iaknoh;
use iaknoh_repeat::update_iaknoh_repeat;
use noh_repeat::update_noh_repeat;
use range::update_range;
use range_or_iaknoh_repeat::update_range_or_iaknoh_repeat;
use range_or_noh_repeat::update_range_or_noh_repeat;

/// 生成器で用いる更新関数のための型
/// GeneratorWithAnnealingのUpdateを実装
#[derive(Debug, Clone)]
pub struct StdUpdate;

/// Updateの実装
impl Update<StdScoreProp, Shift, ShiftState, DayState> for StdUpdate {
    fn generate<'a, R: Rng>(
        &self,
        name: &str,
        schedule_config: &'a ScheduleConfig,
    ) -> anyhow::Result<Box<dyn FnMut(&Schedule, &mut R) -> Schedule + 'a>> {
        let schedule_state = &schedule_config.day.schedule_states;
        match name {
            "update_iaknoh_repeat" => Ok(Box::new(move |schedule, rng| {
                update_iaknoh_repeat(schedule_config, schedule_state, schedule, rng)
            })),
            "update_iaknoh" => Ok(Box::new(move |schedule, rng| {
                update_iaknoh(schedule_config, schedule_state, schedule, rng)
            })),
            "update_noh_repeat" => Ok(Box::new(move |schedule, rng| {
                update_noh_repeat(schedule_config, schedule_state, schedule, rng)
            })),
            "update_iak_safe" => Ok(Box::new(move |schedule, rng| {
                update_iak_safe(schedule_config, schedule_state, schedule, rng)
            })),
            "update_range" => Ok(Box::new(move |schedule, rng| {
                update_range(schedule_config, schedule_state, schedule, rng)
            })),
            "update_range_or_noh_repeat" => Ok(Box::new(move |schedule, rng| {
                update_range_or_noh_repeat(schedule_config, schedule_state, schedule, rng)
            })),
            "update_range_or_iaknoh_repeat" => Ok(Box::new(move |schedule, rng| {
                update_range_or_iaknoh_repeat(schedule_config, schedule_state, schedule, rng)
            })),
            _ => Err(anyhow::anyhow!(
                "Failed to generate update function {}",
                name
            )),
        }
    }
}
