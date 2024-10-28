//! config読み込みのモジュール

use std::fs::{read_to_string};
use std::io;

use crate::kinmu_lib::types::{
    ScheduleProp,
    AnnealingConfig,
    Shift,
    Staff,
    NGList,
    Days,
    DayState,
    Schedule,
    ScoreProp,
    ScheduleState,
    ShiftState,
    FillConfig,
};



type FilePath = String;

pub fn load_main_config(path: &FilePath) -> io::Result<Vec<FilePath>> {

    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    let ans = ss[1].lines().map(|s| s.to_string()).collect();

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
    ss
}

/// 勤務表で使う値を読み込む
pub fn load_config(path: &str) -> io::Result<(ScheduleProp, Vec<FilePath>, FillConfig)> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    let schedule = read_schedule(&ss[7])?;

    let buffer = read_usize(&ss[6])?;

    let hp = ScheduleProp {
        staff: read_staff(&ss[1])?,
        ng_list: read_ng_list(&ss[2])?,
        staff_count: read_usize(&ss[3])?,
        day_count: read_usize(&ss[4])?,
        days: read_days(&ss[5])?,
        buffer: buffer,
        request: schedule.clone(),
        schedule_st: make_schedule_state(&schedule, buffer),
        i_staff_count: read_isizes(&ss[8])?,
        score_props: read_score_props(&ss[12])?,
    };
    let fs = ss[11].lines().map(|s| s.to_string()).collect();
    let fc = FillConfig {
        name: ss[9].clone(), 
        seed: read_usize(&ss[10])?,
    };

    Ok((hp, fs, fc))
}

/// 焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> io::Result<AnnealingConfig> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    let ac = AnnealingConfig {
        step: read_usize(&ss[1])?,
        seed: read_usize(&ss[2])?,
        score_props: read_score_props(&ss[3])?,
        update_func: ss[4].clone(),
        max_temp: read_float(&ss[5])?,
        min_temp: read_float(&ss[6])?,
    };

    Ok(ac)
}

/// ファイルを読み込んで文字列の行ごとの配列を返す関数
fn read_contents(path: &str) -> io::Result<Vec<String>> {

    // ファイルの全文をStringとして読み込む
    let contents = read_to_string(path)?;

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



fn read_usize(text: &str) -> io::Result<usize> {
    let ans: usize = text.parse::<usize>().unwrap();
    Ok(ans)
}

// fn read_usizes(text: &str) -> io::Result<Vec<usize>> {
//     Ok(text.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect())
// }

fn read_isize(text: &str) -> io::Result<isize> {
    let ans: isize = text.parse::<isize>().unwrap();
    Ok(ans)
}

fn read_isizes(text: &str) -> io::Result<Vec<isize>> {
    Ok(text.split_whitespace().map(|x| x.parse::<isize>().unwrap()).collect())
}

fn read_float(text: &str) -> io::Result<f32> {
    let ans: f32 = text.parse::<f32>().unwrap();
    Ok(ans)
}

fn read_float_pair(text: &str) -> io::Result<(f32, f32)> {
    let ns: Vec<f32> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    Ok((ns[0], ns[1]))
}

fn read_isize_float(text: &str) -> io::Result<(isize, f32)> {
    let ns: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    let i = ns[0].parse::<isize>().unwrap();
    let f = ns[1].parse::<f32>().unwrap();
    Ok((i, f))
}

fn read_isize_isize_float(text: &str) -> io::Result<(isize, isize, f32)> {
    let ns: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    let i1 = ns[0].parse::<isize>().unwrap();
    let i2 = ns[1].parse::<isize>().unwrap();
    let f = ns[2].parse::<f32>().unwrap();
    Ok((i1, i2, f))
}

fn read_shift_float(text: &str) -> io::Result<(Shift, f32)> {
    let ns: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    let s = ns[0].parse::<Shift>().unwrap();
    let f = ns[1].parse::<f32>().unwrap();
    Ok((s, f))
}

fn read_daystate_isize_float(text: &str) -> io::Result<(DayState, isize, f32)> {
    let ns: Vec<_> = text
        .trim_matches(|c| c == '(' || c == ')')
        .split(',')
        .collect();
    let d = ns[0].parse::<DayState>().unwrap();
    let i = ns[1].parse::<isize>().unwrap();
    let f = ns[2].parse::<f32>().unwrap();
    Ok((d, i, f))
}

fn read_staff(text: &str) -> io::Result<Vec<Staff>> {
    let mut staff: Vec<Staff> = Vec::new();
    for line in text.lines() {
        let a_staff = read_a_staff(&line)?;
        staff.push(a_staff);
    }
    Ok(staff)
}

fn read_a_staff(text: &str) -> io::Result<Staff> {
    // TODO: もうちょっと安全にアクセスしたい
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
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

fn read_ng_list(text: &str) -> io::Result<NGList> {
    let mut ans: NGList = Vec::new();
    for line in text.lines() {
        let a: Vec<usize> = line.split_whitespace().map(|x| read_usize(x).unwrap()).collect();
        ans.push((a[0], a[1]));
    }
    Ok(ans)
}

fn read_days(text: &str) -> io::Result<Days> {
    Ok(text.chars().map(|c| match c {
        'W' => Ok(DayState::Weekday),
        'H' => Ok(DayState::Holiday),
        'F' => Ok(DayState::Bath),
        '2' => Ok(DayState::Bath2),
        'G' => Ok(DayState::Weight),
        _ => Err("MATCH sinai DAYST desu!!!"),
    }.unwrap()).collect())
}

fn read_schedule(text: &str) -> io::Result<Schedule> {
    let mut ans: Schedule = Vec::new();
    for line in text.lines() {
        let a: Vec<Shift> = line.chars().map(|c| match c {
            'N' => Ok(Shift::N),
            'K' => Ok(Shift::K),
            'I' => Ok(Shift::I),
            'A' => Ok(Shift::A),
            'O' => Ok(Shift::O),
            'H' => Ok(Shift::H),
            'Y' => Ok(Shift::Y),
            'D' => Ok(Shift::D),
            'U' => Ok(Shift::U),
            ' ' => Ok(Shift::U),
            _ => Err("MATCH sinai WAKU desu!!!")
        }.unwrap()).collect();
        ans.push(a);
    }
    Ok(ans)
}

fn read_score_props(text: &str) -> io::Result<Vec<ScoreProp>> {
    let mut ans: Vec<ScoreProp> = Vec::new();
    for line in text.lines() {
        ans.push(read_score_prop(&line)?);
    }
    Ok(ans)
}

fn read_score_prop(text: &str) -> io::Result<ScoreProp> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let prop: ScoreProp = match (words[0], words[1]) {
        ("IAKpattern", p) => ScoreProp::IAKpattern(read_float(p)?),
        ("KIApattern", p) => ScoreProp::KIApattern(read_float(p)?),
        ("KNIApattern", p) => ScoreProp::KNIApattern(read_float(p)?),
        ("NNIApattern", p) => ScoreProp::NNIApattern(read_float(p)?),
        ("ONpattern", p) => ScoreProp::ONpattern(read_float(p)?),
        ("NHpattern", p) => ScoreProp::NHpattern(read_float(p)?),
        ("OHpattern", p) => ScoreProp::OHpattern(read_float(p)?),
        ("WorkingDayStreak4", p) => ScoreProp::WorkingDayStreak4(read_float_pair(p)?),
        ("WorkingDayStreak5", p) => ScoreProp::WorkingDayStreak5(read_float_pair(p)?),
        ("WorkingDayStreak6", p) => ScoreProp::WorkingDayStreak6(read_float_pair(p)?),
        ("HolidayReward", p) => ScoreProp::HolidayReward(read_float(p)?),
        ("Need2Holidays", p) => ScoreProp::Need2Holidays(read_float(p)?),
        ("Need2HolidaysNoBf", p) => ScoreProp::Need2HolidaysNoBf(read_float(p)?),
        ("OHBalance", p) => ScoreProp::OHBalance(read_float(p)?),
        ("ShiftHalfBalance", p) => ScoreProp::ShiftHalfBalance(read_shift_float(p)?),
        ("KDayCount", p) => ScoreProp::KDayCount(read_float(p)?),
        ("IDayCount", p) => ScoreProp::IDayCount(read_float(p)?),
        ("ODayCount", p) => ScoreProp::ODayCount(read_float(p)?),
        ("HDayCount", p) => ScoreProp::HDayCount(read_float(p)?),
        ("Fair", p) => ScoreProp::Fair(read_usize(p)?),
        ("IStaffCount", p) => ScoreProp::IStaffCount(read_float(p)?),
        ("NStaffCount", p) => ScoreProp::NStaffCount(read_daystate_isize_float(p)?),
        ("OStaffCount", p) => ScoreProp::OStaffCount(read_isize_float(p)?),
        ("HStaffCount", p) => ScoreProp::HStaffCount(read_isize_float(p)?),
        ("NGPair", p) => ScoreProp::NGPair(read_float(p)?),
        ("LeaderAbility", p) => ScoreProp::LeaderAbility(read_isize_float(p)?),
        ("IAloneAbility", p) => ScoreProp::IAloneAbility(read_isize_float(p)?),
        ("IAloneBeforeBath", p) => ScoreProp::IAloneBeforeBath(read_float(p)?),
        ("RoomLeaderAbility", p) => ScoreProp::RoomLeaderAbility(read_isize_isize_float(p)?),
        ("NoSamePair3", p) => ScoreProp::NoSamePair3(read_float(p)?),
        ("NoSamePair2", p) => ScoreProp::NoSamePair2(read_float(p)?),
        ("NoUndef", p) => ScoreProp::NoUndef(read_float(p)?),
        (s, p) => {println!("MATCH SINAI SCORE PROP DESU!!!!: {} {}",s,p); ScoreProp::NoUndef(0.0)},
    };
    Ok(prop) //マッチしない場合、Errを返してリストに追加しないようにしたいかも
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