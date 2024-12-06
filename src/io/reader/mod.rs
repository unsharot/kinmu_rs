//! config読み込みのモジュール

mod common;
mod seed;
mod type_reader;

use common::*;
use type_reader::*;

use std::fs;

use crate::kinmu_lib::types::{
    AnnealingConfig, Days, FillConfig, Schedule, ScheduleProp, ScheduleState, ScoreProp, Shift,
    ShiftState, Staff,
};

type FilePath = String;

pub fn load_main_config(path: &str) -> Result<Vec<FilePath>, String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(1, &ss, "項目が足りません", "項目が余分です")?;

    let ans = ss[0].lines().map(|s| s.to_string()).collect();

    Ok(ans)
}

/// 勤務表で使う値を読み込む
pub fn load_schedule_config(
    path: &str,
) -> Result<(ScheduleProp, Vec<FilePath>, FillConfig), String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(13, &ss, "項目が足りません", "項目が余分です")?;

    let StaffAttributeMapWrapper(staff_attribute_name_index_map) =
        <StaffAttributeMapWrapper>::from_config(&ss[0])?;
    let staff_list = <Vec<Staff>>::from_config(&ss[1])?;
    let NGListWrapper(ng_list) = <NGListWrapper>::from_config(&ss[2])?;
    let staff_count = <usize>::from_config(&ss[3])?;
    let day_count = <usize>::from_config(&ss[4])?;
    let days = <Days>::from_config(&ss[5])?;
    let buffer = <usize>::from_config(&ss[6])?;
    let ScheduleWrapper(schedule) = <ScheduleWrapper>::from_config(&ss[7])?;
    let DayAttributeWrapper(day_attributes) = <DayAttributeWrapper>::from_config(&ss[8])?;

    for attribute in day_attributes.clone().into_values() {
        check_len(
            day_count,
            &attribute,
            "DayAttributeが日数分ありません",
            "DayAttributeが日数分を超過しています",
        )?;
    }
    check_len(
        staff_count,
        &staff_list,
        "職員リストが設定した職員数だけありません",
        "職員リストが設定した職員数を超過しています",
    )?;
    check_len(
        day_count,
        &days,
        "DayStateが設定した日数だけありません",
        "DayStateが設定した日数を超過しています",
    )?;
    check_len(
        staff_count,
        &schedule,
        "スケジュールが職員数分ありません",
        "スケージュールが職員数を超過しています",
    )?;
    for r in 0..staff_count {
        check_len(
            day_count,
            &schedule[r],
            "スケジュールが日数分ありません",
            "スケージュールが日数を超過しています",
        )?;
    }

    let hp = ScheduleProp {
        staff_list: staff_list,
        ng_list: ng_list,
        staff_count: staff_count,
        day_count: day_count,
        days: days,
        buffer: buffer,
        request: schedule.clone(),
        schedule_st: make_schedule_state(&schedule, buffer),
        day_attributes: day_attributes,
        staff_attribute_map: staff_attribute_name_index_map,
        score_props: <Vec<ScoreProp>>::from_config(&ss[12])?,
    };
    let fs = ss[11].lines().map(|s| s.to_string()).collect();
    let fc = FillConfig {
        name: ss[9].clone(),
        rng: seed::gen_rng_from_seed(<usize>::from_config(&ss[10])?),
    };

    Ok((hp, fs, fc))
}

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> Result<AnnealingConfig, String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(5, &ss, "項目が足りません", "項目が余分です")?;

    let TempWrapper(tmax, tmin) = <TempWrapper>::from_config(&ss[4])?;

    let ac = AnnealingConfig {
        step: <usize>::from_config(&ss[0])?,
        rng: seed::gen_rng_from_seed(<usize>::from_config(&ss[1])?),
        score_props: <Vec<ScoreProp>>::from_config(&ss[2])?,
        update_func: ss[3].clone(),
        max_temp: tmax,
        min_temp: tmin,
    };

    Ok(ac)
}

/// ファイルを読み込んで文字列の行ごとの配列を返す関数
fn read_contents(path: &str) -> Result<Vec<String>, String> {
    // ファイルの全文をStringとして読み込む
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

    // 成形して行ごとのVec<String>にする
    let mut ans: Vec<String> = Vec::new();
    for line in contents.lines() {
        // コメントを除去
        let cleaned_line = match line.find('#') {
            Some(index) => &line[..index],
            None => &line,
        };
        // 空白の行を除去
        if cleaned_line != "" {
            ans.push(cleaned_line.to_string());
        }
    }

    Ok(ans)
}

/// フィールドごとに区切る
fn sep_by_fields(contents: &Vec<String>) -> Vec<String> {
    let mut temp: Vec<String> = Vec::new();
    let mut ss: Vec<String> = Vec::new();
    for line in contents {
        if line.trim().ends_with(":") {
            ss.push(temp.join("\n"));
            temp = Vec::new();
        } else {
            temp.push(line.to_string());
        }
    }
    ss.push(temp.join("\n"));
    ss[1..].to_vec()
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
