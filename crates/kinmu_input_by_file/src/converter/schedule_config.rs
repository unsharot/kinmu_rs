//! schedule_configを変換する関数を提供するモジュール

use std::collections::HashMap;

use anyhow::Context;

use super::super::reader::types::{RawAttributeTable, RawScheduleConfig};
use kinmu_model::{
    DayAttributeName, DayConfig, FillConfig, ResultConfig, Schedule, ScheduleConfig, ScheduleState,
    ScoreFilter, ScoreFunction, Staff, StaffAttributeNameIndexMap, StaffConfig,
};

use super::util::parser::*;

/// RawScheduleConfigをScheduleConfigに変換する
/// annealing_configsフィールドは空なので、あとから設定しなおす
pub fn convert_schedule_config<SP: FromConfig, S: FromConfig + MapState<SS>, SS, DS: FromConfig>(
    config: RawScheduleConfig,
) -> anyhow::Result<ScheduleConfig<SP, S, SS, DS>> {
    let schedule = config
        .day
        .requested_schedule
        .iter()
        .map(|s| {
            <CharVecWrapper<S>>::from_config(s)
                .map(|w| w.0)
                .with_context(|| format!("Failed to parse schedule row \"{}\"", s))
        })
        .collect::<anyhow::Result<Schedule<S>>>()
        .context("Failed to parse day.requested_schedule")?;

    let staff_config = StaffConfig {
        attribute_map: make_staff_attribute_map(config.staff.attributes),
        list: config
            .staff
            .list
            .into_iter()
            .map(|x| Staff {
                name: x.name,
                attributes: x.attributes,
            })
            .collect(),
        ng_list: config
            .staff
            .ng_list
            .iter()
            .map(|x| (x.from, x.to))
            .collect(),
        count: config.staff.count,
    };

    let day_config = DayConfig {
        count: config.day.day_count,
        buffer_count: config.day.buffer_count,
        days: <CharVecWrapper<DS>>::from_config(&config.day.states)
            .context("Failed to parse day.states")?
            .0,
        schedule_states: make_schedule_state(&schedule, config.day.buffer_count),
        requested_schedule: schedule,
        attributes: make_day_attributes(config.day.attributes),
    };

    let fill_config = FillConfig {
        name: config.fill.function,
        seed: config.fill.seed,
    };

    let result_config = ResultConfig {
        score_functions: config
            .result
            .score_functions
            .into_iter()
            .map(|sf| {
                Ok(ScoreFunction {
                    scores: sf
                        .scores
                        .iter()
                        .map(|s| <SP>::from_config(s))
                        .collect::<anyhow::Result<Vec<SP>>>()
                        .with_context(|| {
                            format!("Failed to parse score_function named {}", &sf.display_name)
                        })?,
                    display_name: sf.display_name,
                    filter: sf.filter.map(|f| ScoreFilter {
                        low_pass: f.low_pass,
                        high_pass: f.high_pass,
                    }),
                })
            })
            .collect::<anyhow::Result<Vec<ScoreFunction<SP>>>>()
            .context("Failed to parse result.score_functions")?,
    };

    let schedule_config: ScheduleConfig<SP, S, SS, DS> = ScheduleConfig {
        staff: staff_config,
        day: day_config,
        fill: fill_config,
        annealing_configs: Default::default(),
        result: result_config,
    };

    Ok(schedule_config)
}

fn make_day_attributes(attributes: Vec<RawAttributeTable>) -> HashMap<DayAttributeName, Vec<i32>> {
    let mut ans = HashMap::new();
    for att in attributes {
        ans.insert(att.name, att.values);
    }
    ans
}

fn make_staff_attribute_map(attributes: Vec<String>) -> StaffAttributeNameIndexMap {
    let mut name_to_index = HashMap::new();
    for (i, name) in attributes.iter().enumerate() {
        name_to_index.insert(name.to_string(), i);
    }
    StaffAttributeNameIndexMap {
        names: attributes,
        name_to_index,
    }
}

/// Shiftに実装するためのトレイト
/// 入力された場所を変化させてよいかを記録するShiftStateをrequested_scheduleから生成する際に用いる
pub trait MapState<SS> {
    /// バッファの範囲に対して適応するデフォルトのShiftStateを指定
    const BUFFER_CASE: SS;
    /// SSに変換する
    fn to_state(&self) -> SS;
}

/// ScheduleStateを与えられた表から生成する
/// シフトとして用いるSにはMapStateを要求
fn make_schedule_state<S: MapState<SS>, SS>(
    schedule: &Schedule<S>,
    buffer: usize,
) -> ScheduleState<SS> {
    let mut ans: ScheduleState<SS> = Vec::new();
    for line in schedule {
        ans.push(
            line.iter()
                .enumerate()
                .map(|(i, shift)| {
                    if i < buffer {
                        S::BUFFER_CASE
                    } else {
                        shift.to_state()
                    }
                })
                .collect(),
        );
    }
    ans
}
