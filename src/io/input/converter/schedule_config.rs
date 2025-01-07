//! schedule_configを読み込むモジュール

use std::collections::HashMap;

use crate::io::input::reader::types::RawScheduleConfig;
use crate::kinmu_lib::types::{
    DayAttributeName, Days, FilePath, FillConfig, Schedule, ScheduleProp, ScheduleState, ScoreProp,
    Shift, ShiftState, Staff, StaffAttributeNameIndexMap,
};

use super::util::checker;
use super::util::parser::*;

/// 勤務表で使う値を読み込む
pub fn convert_schedule_config(
    config: RawScheduleConfig,
) -> Result<(ScheduleProp, Vec<FilePath>, FillConfig), String> {
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
        //TODO ここ要修正 Optionのまま渡してgeneratorで処理したい
        seed: match config.fill.seed {
            Some(x) => x,
            None => 0,
        },
    };
    checker::check_schedule_prop(&schedule_prop)?;
    Ok((schedule_prop, annealing_file_paths, fill_config))
}

fn make_schedule(config: &RawScheduleConfig) -> Result<Schedule, String> {
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
