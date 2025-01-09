//! schedule_configを読み込むモジュール

use std::collections::HashMap;

use crate::io::input::reader::types::RawScheduleConfig;
use crate::kinmu_lib::types::{
    DayAttributeName, DayConfig, Days, FilePath, FillConfig, ResultConfig, Schedule,
    ScheduleConfig, ScheduleState, ScoreProp, Shift, ShiftState, Staff, StaffAttributeNameIndexMap,
    StaffConfig,
};

use super::util::checker;
use super::util::parser::*;

/// 勤務表で使う値を読み込む
pub fn convert_schedule_config(
    config: RawScheduleConfig,
) -> Result<(ScheduleConfig, Vec<FilePath>, FillConfig), String> {
    let schedule = config
        .day
        .requested_schedule
        .iter()
        .map(|s| <ScheduleRowWrapper>::from_config(s).map(|w| w.value))
        .collect::<Result<Schedule, String>>()?;

    let staff_config = StaffConfig {
        attribute_map: make_staff_attribute_map(&config),
        list: config
            .staff
            .list
            .iter()
            .map(|x| Staff {
                name: x.name.clone(),
                attributes: x.attributes.clone(),
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
        days: Days::from_config(&config.day.states)?,
        schedule_states: make_schedule_state(&schedule, config.day.buffer_count),
        requested_schedule: schedule,
        attributes: make_day_attributes(&config),
    };

    let result_config = ResultConfig {
        score_props: config
            .result
            .score_functions
            .iter()
            .map(|s| <ScoreProp>::from_config(s))
            .collect::<Result<Vec<ScoreProp>, String>>()?,
    };

    let schedule_config: ScheduleConfig = ScheduleConfig {
        staff: staff_config,
        day: day_config,
        result: result_config,
    };

    let annealing_file_paths = config.annealing.config_paths;

    let fill_config = FillConfig {
        name: config.fill.function,
        seed: config.fill.seed,
    };

    checker::check_schedule_config(&schedule_config)?;

    Ok((schedule_config, annealing_file_paths, fill_config))
}

fn make_day_attributes(config: &RawScheduleConfig) -> HashMap<DayAttributeName, Vec<i32>> {
    let mut ans = HashMap::new();
    for att in &config.day.attributes {
        ans.insert(att.name.clone(), att.values.clone());
    }
    ans
}

fn make_staff_attribute_map(config: &RawScheduleConfig) -> StaffAttributeNameIndexMap {
    let mut name_to_index = HashMap::new();
    for (i, name) in config.staff.attributes.iter().enumerate() {
        name_to_index.insert(name.to_string(), i);
    }
    StaffAttributeNameIndexMap {
        names: config.staff.attributes.clone(),
        name_to_index,
    }
}

fn make_schedule_state(schedule: &Schedule, buffer: usize) -> ScheduleState {
    let mut ans: ScheduleState = Vec::new();
    for line in schedule {
        ans.push(
            line.iter()
                .enumerate()
                .map(|(i, shift)| {
                    if i < buffer {
                        ShiftState::Absolute
                    } else {
                        match shift {
                            Shift::U => ShiftState::Random,
                            _ => ShiftState::Absolute,
                        }
                    }
                })
                .collect(),
        );
    }
    ans
}
