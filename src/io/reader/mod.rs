//! config読み込みのモジュール

mod common;
mod type_reader;
mod seed;

use type_reader::*;
use common::*;

use std::fs;

use crate::kinmu_lib::types::{
    ScheduleProp,
    AnnealingConfig,
    Shift,
    Schedule,
    ScheduleState,
    ShiftState,
    FillConfig,
};


type FilePath = String;


pub fn load_main_config(path: &FilePath) -> Result<Vec<FilePath>, String> {

    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(1, &ss, "項目が足りません", "項目が余分です")?;

    let ans = ss[0].lines().map(|s| s.to_string()).collect();

    Ok(ans)
}

/// 勤務表で使う値を読み込む
pub fn load_schedule_config(path: &str) -> Result<(ScheduleProp, Vec<FilePath>, FillConfig), String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(12, &ss, "項目が足りません", "項目が余分です")?;

    let staff_list = read_staff_list(&ss[0])?;
    let ng_list = read_ng_list(&ss[1])?;
    let staff_count = read_usize(&ss[2])?;
    let day_count = read_usize(&ss[3])?;
    let days = read_days(&ss[4])?;
    let buffer = read_usize(&ss[5])?;
    let schedule = read_schedule(&ss[6])?;
    let i_staff_count = read_isizes(&ss[7])?;

    check_len(day_count - buffer, &i_staff_count, "夜勤の人数が日数分ありません", "夜勤の人数が日数分を超過しています")?;
    check_len(staff_count, &staff_list, "職員リストが設定した職員数だけありません", "職員リストが設定した職員数を超過しています")?;
    check_len(day_count, &days, "DayStateが設定した日数だけありません", "DayStateが設定した日数を超過しています")?;
    check_len(staff_count, &schedule, "スケジュールが職員数分ありません", "スケージュールが職員数を超過しています")?;
    for r in 0..staff_count {
        check_len(day_count, &schedule[r], "スケジュールが日数分ありません", "スケージュールが日数を超過しています")?;
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
        i_staff_count: i_staff_count,
        score_props: read_score_props(&ss[11])?,
    };
    let fs = ss[10].lines().map(|s| s.to_string()).collect();
    let fc = FillConfig {
        name: ss[8].clone(), 
        rng: seed::gen_rng_from_seed(read_usize(&ss[9])?),
    };

    Ok((hp, fs, fc))
}

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> Result<AnnealingConfig, String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(5, &ss, "項目が足りません", "項目が余分です")?;

    let (tmax, tmin) = read_temp(&ss[4])?;

    let ac = AnnealingConfig {
        step: read_usize(&ss[0])?,
        rng: seed::gen_rng_from_seed(read_usize(&ss[1])?),
        score_props: read_score_props(&ss[2])?,
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
        ans.push(line.iter().enumerate().map(|(i, shift)|
            if i < buffer {
                ShiftState::Absolute
            } else {
                match shift {
                    Shift::U => ShiftState::Random,
                    _ => ShiftState::Absolute,
                }
            }
        ).collect());
    }
    ans
}