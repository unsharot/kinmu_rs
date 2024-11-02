//! config読み込みのモジュール

use std::fs::{read_to_string};

use crate::kinmu_lib::types::{
    ScheduleProp,
    AnnealingConfig,
    Shift,
    Staff,
    NGList,
    NG,
    Days,
    DayState,
    Schedule,
    ScoreProp,
    ScheduleState,
    ShiftState,
    FillConfig,
};



type FilePath = String;

pub fn load_main_config(path: &FilePath) -> Result<Vec<FilePath>, String> {

    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(1, &ss, "項目が足りません")?;

    let ans = ss[0].lines().map(|s| s.to_string()).collect();

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

/// 勤務表で使う値を読み込む
pub fn load_config(path: &str) -> Result<(ScheduleProp, Vec<FilePath>, FillConfig), String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(12, &ss, "項目が足りません")?;

    let schedule = read_schedule(&ss[6])?;

    let buffer = read_usize(&ss[5])?;

    let hp = ScheduleProp {
        staff: read_staff(&ss[0])?,
        ng_list: read_ng_list(&ss[1])?,
        staff_count: read_usize(&ss[2])?,
        day_count: read_usize(&ss[3])?,
        days: read_days(&ss[4])?,
        buffer: buffer,
        request: schedule.clone(),
        schedule_st: make_schedule_state(&schedule, buffer),
        i_staff_count: read_isizes(&ss[7])?,
        score_props: read_score_props(&ss[11])?,
    };
    let fs = ss[10].lines().map(|s| s.to_string()).collect();
    let fc = FillConfig {
        name: ss[8].clone(), 
        seed: read_usize(&ss[9])?,
    };

    Ok((hp, fs, fc))
}

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> Result<AnnealingConfig, String> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    check_len(3, &ss, "項目が足りません")?;

    let (tmax, tmin) = read_temp(&ss[4])?;

    let ac = AnnealingConfig {
        step: read_usize(&ss[0])?,
        seed: read_usize(&ss[1])?,
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
    let contents = read_to_string(path).map_err(|e| e.to_string())?;

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



fn read_usize(text: &str) -> Result<usize, String> {
    let ans: usize = text.parse::<usize>().map_err(|e| e.to_string())?;
    Ok(ans)
}

// fn read_usizes(text: &str) -> Result<Vec<usize>, String> {
//     text.split_whitespace().map(|x| x.parse::<usize>().map_err(|e| e.to_string())).collect()
// }

fn read_isize(text: &str) -> Result<isize, String> {
    let ans: isize = text.parse::<isize>().map_err(|e| e.to_string())?;
    Ok(ans)
}

fn read_isizes(text: &str) -> Result<Vec<isize>, String> {
    text.split_whitespace().map(|x| x.parse::<isize>().map_err(|e| e.to_string())).collect()
}

fn read_float(text: &str) -> Result<f32, String> {
    let ans: f32 = text.parse::<f32>().map_err(|e| e.to_string())?;
    Ok(ans)
}

fn read_float_pair(text: &str) -> Result<(f32, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    let f1 = words[0].parse::<f32>().map_err(|e| e.to_string())?;
    let f2 = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((f1, f2))
}

fn read_isize_float(text: &str) -> Result<(isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    let i = words[0].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((i, f))
}

fn read_isize_isize_float(text: &str) -> Result<(isize, isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(3, &words, "Needs 3 fields, but not enough.")?;
    let i1 = words[0].parse::<isize>().map_err(|e| e.to_string())?;
    let i2 = words[1].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[2].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((i1, i2, f))
}

fn read_shift_float(text: &str) -> Result<(Shift, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    let s = words[0].parse::<Shift>().map_err(|e| e.to_string())?;
    let f = words[1].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((s, f))
}

fn read_daystate_isize_float(text: &str) -> Result<(DayState, isize, f32), String> {
    let words: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    check_len(3, &words, "Needs 3 fields, but not enough.")?;
    let d = words[0].parse::<DayState>().map_err(|e| e.to_string())?;
    let i = words[1].parse::<isize>().map_err(|e| e.to_string())?;
    let f = words[2].parse::<f32>().map_err(|e| e.to_string())?;
    Ok((d, i, f))
}

fn read_staff(text: &str) -> Result<Vec<Staff>, String> {
    let mut staff: Vec<Staff> = Vec::new();
    for line in text.lines() {
        let a_staff = read_a_staff(&line)?;
        staff.push(a_staff);
    }
    Ok(staff)
}

fn read_a_staff(text: &str) -> Result<Staff, String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(6, &words, "Needs 6 fields, but not enough.")?;
    let worker: Staff = Staff {
        name: words[5].clone(),
        ability: read_isize(&words[0])?,
        k_day_count: read_isize(&words[1])?,
        i_day_count: read_isize(&words[2])?,
        o_day_count: read_isize(&words[3])?,
        h_day_count: read_isize(&words[4])?,
    };
    Ok(worker)
}

fn read_ng_list(text: &str) -> Result<NGList, String> {
    let mut ans: NGList = Vec::new();
    for line in text.lines() {
        let ng = read_ng(&line)?;
        ans.push(ng);
    }
    Ok(ans)
}

fn read_ng(text: &str) -> Result<NG, String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    let id1 = read_usize(&words[0])?;
    let id2 = read_usize(&words[1])?;
    Ok((id1, id2))
}

fn read_temp(text: &str) -> Result<(f32, f32), String> {
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    let id1 = read_float(&words[0])?;
    let id2 = read_float(&words[1])?;
    Ok((id1, id2))
}

fn read_days(text: &str) -> Result<Days, String> {
    let mut ans: Days = Vec::new();
    for c in text.chars() {
        ans.push(c.to_string().parse::<DayState>()?);
    }
    Ok(ans)
}

fn read_schedule(text: &str) -> Result<Schedule, String> {
    let mut ans: Schedule = Vec::new();
    for line in text.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_string().parse::<Shift>()?);
        }
        ans.push(row);
    }
    Ok(ans)
}

fn read_score_props(text: &str) -> Result<Vec<ScoreProp>, String> {
    let mut ans: Vec<ScoreProp> = Vec::new();
    for line in text.lines() {
        ans.push(read_score_prop(&line)?);
    }
    Ok(ans)
}

fn read_score_prop(text: &str) -> Result<ScoreProp, String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    check_len(2, &words, "Needs 2 fields, but not enough.")?;
    match (words[0], words[1]) {
        ("IAKpattern", p) => Ok(ScoreProp::IAKpattern(read_float(p)?)),
        ("KIApattern", p) => Ok(ScoreProp::KIApattern(read_float(p)?)),
        ("KNIApattern", p) => Ok(ScoreProp::KNIApattern(read_float(p)?)),
        ("NNIApattern", p) => Ok(ScoreProp::NNIApattern(read_float(p)?)),
        ("ONpattern", p) => Ok(ScoreProp::ONpattern(read_float(p)?)),
        ("NHpattern", p) => Ok(ScoreProp::NHpattern(read_float(p)?)),
        ("OHpattern", p) => Ok(ScoreProp::OHpattern(read_float(p)?)),
        ("WorkingDayStreak4", p) => Ok(ScoreProp::WorkingDayStreak4(read_float_pair(p)?)),
        ("WorkingDayStreak5", p) => Ok(ScoreProp::WorkingDayStreak5(read_float_pair(p)?)),
        ("WorkingDayStreak6", p) => Ok(ScoreProp::WorkingDayStreak6(read_float_pair(p)?)),
        ("HolidayReward", p) => Ok(ScoreProp::HolidayReward(read_float(p)?)),
        ("Need2Holidays", p) => Ok(ScoreProp::Need2Holidays(read_float(p)?)),
        ("Need2HolidaysNoBf", p) => Ok(ScoreProp::Need2HolidaysNoBf(read_float(p)?)),
        ("OHBalance", p) => Ok(ScoreProp::OHBalance(read_float(p)?)),
        ("ShiftHalfBalance", p) => Ok(ScoreProp::ShiftHalfBalance(read_shift_float(p)?)),
        ("KDayCount", p) => Ok(ScoreProp::KDayCount(read_float(p)?)),
        ("IDayCount", p) => Ok(ScoreProp::IDayCount(read_float(p)?)),
        ("ODayCount", p) => Ok(ScoreProp::ODayCount(read_float(p)?)),
        ("HDayCount", p) => Ok(ScoreProp::HDayCount(read_float(p)?)),
        ("Fair", p) => Ok(ScoreProp::Fair(read_usize(p)?)),
        ("IStaffCount", p) => Ok(ScoreProp::IStaffCount(read_float(p)?)),
        ("NStaffCount", p) => Ok(ScoreProp::NStaffCount(read_daystate_isize_float(p)?)),
        ("OStaffCount", p) => Ok(ScoreProp::OStaffCount(read_isize_float(p)?)),
        ("HStaffCount", p) => Ok(ScoreProp::HStaffCount(read_isize_float(p)?)),
        ("NGPair", p) => Ok(ScoreProp::NGPair(read_float(p)?)),
        ("LeaderAbility", p) => Ok(ScoreProp::LeaderAbility(read_isize_float(p)?)),
        ("IAloneAbility", p) => Ok(ScoreProp::IAloneAbility(read_isize_float(p)?)),
        ("IAloneBeforeBath", p) => Ok(ScoreProp::IAloneBeforeBath(read_float(p)?)),
        ("RoomLeaderAbility", p) => Ok(ScoreProp::RoomLeaderAbility(read_isize_isize_float(p)?)),
        ("NoSamePair3", p) => Ok(ScoreProp::NoSamePair3(read_float(p)?)),
        ("NoSamePair2", p) => Ok(ScoreProp::NoSamePair2(read_float(p)?)),
        ("NoUndef", p) => Ok(ScoreProp::NoUndef(read_float(p)?)),
        (s, p) => Err(format!("Failed to parse ScoreProp: {} {}",s,p))
    }
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


fn check_len<T, E>(l: usize, v: &Vec<T>, error: E) -> Result<(), E> {
    if v.len() < l {
        return Err(error);
    }
    Ok(())
}