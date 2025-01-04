//! config読み込みのモジュール

mod checker;
mod common;
mod toml_reader;
mod type_reader;

use toml_reader::{MainConfig, ScheduleConfig};
use type_reader::*;

use crate::kinmu_lib::types::{
    AnnealingConfig, DayAttributeName, Days, FillConfig, Schedule, ScheduleProp, ScheduleState,
    ScoreProp, Shift, ShiftState, Staff, StaffAttributeNameIndexMap,
};

use std::collections::HashMap;

type FilePath = String;

pub fn load_main_config(path: &str) -> Result<MainConfig, String> {
    Ok(toml_reader::read_main_config(path)?)
}

/// 勤務表で使う値を読み込む
pub fn load_schedule_config(
    path: &str,
) -> Result<(ScheduleProp, Vec<FilePath>, FillConfig), String> {
    let config = toml_reader::read_schedule_config(path)?;

    let schedule = make_schedule(&config)?;

    let schedule_prop: ScheduleProp = ScheduleProp {
        staff_list: config
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
        staff_count: config.staff.count,
        day_count: config.day.day_count,
        days: Days::from_config(&config.day.states)?,
        buffer: config.day.buffer_count,
        request: schedule.clone(),
        schedule_st: make_schedule_state(&schedule, config.day.buffer_count),
        day_attributes: make_day_attributes(&config),
        staff_attribute_map: make_staff_attribute_map(&config),
        score_props: config
            .result
            .score_functions
            .iter()
            .map(|s| <ScoreProp>::from_config(s))
            .collect::<Result<Vec<ScoreProp>, String>>()?,
    };
    let annealing_file_paths = config.annealing.config_paths;
    let fill_config = FillConfig {
        name: config.fill.function,
        seed: config.fill.seed,
    };
    checker::check_schedule_prop(&schedule_prop)?;
    Ok((schedule_prop, annealing_file_paths, fill_config))
}

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> Result<AnnealingConfig, String> {
    let config = toml_reader::read_annealing_config(path)?;

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

fn make_schedule(config: &ScheduleConfig) -> Result<Schedule, String> {
    let mut ans: Schedule = Vec::new();
    for s in &config.day.requested_schedule {
        let mut row = Vec::new();
        for c in s.chars() {
            row.push(c.to_string().parse::<Shift>().map_err(|e| e.to_string())?);
        }
        ans.push(row);
    }
    Ok(ans)
}

fn make_day_attributes(config: &ScheduleConfig) -> HashMap<DayAttributeName, Vec<i32>> {
    let mut ans = HashMap::new();
    for att in &config.day.attributes {
        ans.insert(att.name.clone(), att.values.clone());
    }
    ans
}

fn make_staff_attribute_map(config: &ScheduleConfig) -> StaffAttributeNameIndexMap {
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
