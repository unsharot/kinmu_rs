//! 焼きなましで使う評価関数のモジュール

use super::types::{
    CondWrapper, DayAttributeName, Schedule, ScheduleProp, Score, ScoreProp, Shift,
    StaffAttributeName,
};

use std::collections::HashMap;

pub fn assess_score(
    sps: &mut Vec<ScoreProp>,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) -> Score {
    get_score_list(sps, schedule_prop, schedule).iter().sum()
}

pub fn show_score(
    sps: &mut Vec<ScoreProp>,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) -> String {
    let sl = get_score_list(sps, schedule_prop, schedule);
    let ss: Vec<String> = sps.iter().map(|x| x.to_string()).collect();
    let zipped: Vec<String> = ss
        .iter()
        .zip(sl.iter())
        .map(|(x, y)| x.to_string() + " : " + &y.to_string())
        .collect();
    zipped.join("\n")
}

fn get_score_list(
    sps: &mut Vec<ScoreProp>,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) -> Vec<Score> {
    sps.iter_mut()
        .map(|sp: &mut ScoreProp| get_score(schedule_prop, schedule, sp))
        .collect()
}

fn get_score(schedule_prop: &ScheduleProp, schedule: &Schedule, sp: &mut ScoreProp) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => pattern_general(schedule_prop, schedule, p),
        ScoreProp::PatternFixed(p) => pattern_fixed(schedule_prop, schedule, p),
        ScoreProp::Streak(p) => streak(schedule_prop, schedule, p),
        ScoreProp::Need2Holidays(p) => need_2_holidays(schedule_prop, schedule, p),
        ScoreProp::ShiftsBalance(p) => shifts_balance(schedule_prop, schedule, p),
        ScoreProp::ShiftHalfBalance(p) => shift_half_balance(schedule_prop, schedule, p),
        ScoreProp::ShiftDirPriority(p) => shift_dir_priority(schedule_prop, schedule, p),
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            day_count_regard_staff_attribute(schedule_prop, schedule, p)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            staff_count_regard_day_attribute(schedule_prop, schedule, p)
        }
        ScoreProp::StaffCount(p) => staff_count(schedule_prop, schedule, p),
        ScoreProp::NGPair(p) => ng_pair(schedule_prop, schedule, p),
        ScoreProp::NoSamePair(p) => no_same_pair(schedule_prop, schedule, p),
    }
}

/// 指定したシフトパターンが存在するか判定するスコア
/// ただし、シフトパターンは複数候補を指定可能
/// 配置がかぶる場合、うまく判定されない可能性あり
fn pattern_general(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Vec<Shift>>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut a = 0.0;
        let mut accum = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) {
                if shift_pattern[accum].contains(&schedule[staff][day]) {
                    accum += 1;
                    if accum == shift_pattern.len() {
                        accum = 0;
                        a += *score;
                    }
                } else {
                    accum = 0;
                }
            }
        }
        sum += a;
    }
    sum
}

/// 指定したシフトパターンが存在するか判定するスコア
/// 配置がかぶる場合、うまく判定されない可能性あり
fn pattern_fixed(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut a = 0.0;
        let mut accum = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) {
                if shift_pattern[accum] == schedule[staff][day] {
                    accum += 1;
                    if accum == shift_pattern.len() {
                        accum = 0;
                        a += *score;
                    }
                } else {
                    accum = 0;
                }
            }
        }
        sum += a;
    }
    sum
}

/// 指定したシフトが指定回数連続して存在するか判定するスコア
/// 指定回数+1回連続は1回分としてカウントされる
fn streak(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, target_shifts, streak_count, score): &mut (CondWrapper, Vec<Shift>, isize, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut a = 0.0;
        let mut accum = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) {
                if target_shifts.contains(&schedule[staff][day]) {
                    accum += 1;
                } else {
                    accum = 0;
                }
                if accum >= *streak_count {
                    a += *score;
                    accum = 0;
                }
            }
        }
        sum += a;
    }
    sum
}

/// 休日として指定したシフトの2連休が月最低1回あるか判定するスコア
fn need_2_holidays(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, holidays, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut has_2_holidays = false;
        let mut accum = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) {
                if holidays.contains(&schedule[staff][day]) {
                    accum += 1;
                } else {
                    accum = 0;
                }
                if accum >= 2 {
                    has_2_holidays = true;
                    break;
                }
            }
        }
        if !has_2_holidays {
            sum += *score;
        }
    }
    sum
}

/// 指定した2つのシフト数がスタッフあたりでバランス良いか判定するスコア
fn shifts_balance(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &mut (CondWrapper, Shift, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut cnt1: isize = 0;
        let mut cnt2: isize = 0;
        for staff in 0..schedule_prop.staff_count {
            if cond.eval(staff, day, schedule_prop) {
                if schedule[staff][day] == *shift1 {
                    cnt1 += 1;
                }
                if schedule[staff][day] == *shift2 {
                    cnt2 += 1;
                }
            }
        }
        let d = (cnt1 - cnt2).abs() as Score;
        let a = d * *score;
        sum += a * a;
    }
    sum
}

/// 指定したシフトが月の前後でバランスよく配置されているかを判定するスコア
fn shift_half_balance(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        // 条件を満たすdayの中から中間を探す
        let mut len = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                len += 1;
            }
        }
        let mid = len / 2;

        let mut cf: isize = 0;
        let mut cl: isize = 0;
        let mut i = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                i += 1;
                if i < mid {
                    cf += 1;
                } else {
                    cl += 1;
                }
            }
        }
        let d = (cf - cl).abs() as Score;
        let a = d * *score;
        sum += a * a;
    }
    sum
}

/// 指定したシフトが月の前後どちらにあるほうが良いか設定する
/// Scoreのフィールドが正なら前を優先、負なら後ろを優先
fn shift_dir_priority(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        // 条件を満たすdayの中から中間を探す
        let mut len = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                len += 1;
            }
        }
        let mid = len / 2;

        let mut a = 0.0;
        let mut i = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                i += 1;
                a += *score * ((i as Score) - (mid as Score));
            }
        }
        sum += a;
    }
    sum
}

/// 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア
fn day_count_regard_staff_attribute(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, StaffAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut cnt: isize = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                cnt += 1;
            }
        }
        let cnt_needed = schedule_prop.get_attribute(staff, attribute);
        if cnt_needed != -1 { // 値が-1の場合、任意の数を許すためスコアを増やさない
            let d = (cnt - cnt_needed).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

/// 指定したシフトがDayAttributeで指定した数いない場合に発火するスコア
fn staff_count_regard_day_attribute(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, DayAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut cnt: isize = 0;
        for staff in 0..schedule_prop.staff_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                cnt += 1;
            }
        }
        let cnt_needed = schedule_prop.day_attributes.get(attribute).unwrap()[day];
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * *score;
        sum += a * a;
    }
    sum
}

/// 指定したシフトが指定した数いない場合に発火するスコア
fn staff_count(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, count, score): &mut (CondWrapper, Shift, isize, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut cnt = 0;
        for staff in 0..schedule_prop.staff_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                cnt += 1;
            }
        }
        let d = (cnt - *count).abs() as Score;
        let a = d * *score;
        sum += a * a;
    }
    sum
}

/// NGリストにあるペアがともに指定したシフトなら発火するスコア
fn ng_pair(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut a = 0.0;
        for i in 0..schedule_prop.ng_list.len() {
            let (staff1, staff2) = schedule_prop.ng_list[i];
            if cond.eval(staff1, day, schedule_prop)
                && cond.eval(staff2, day, schedule_prop)
                && schedule[staff1][day] == *shift
                && schedule[staff2][day] == *shift
            {
                a += *score;
            }
        }
        sum += a;
    }
    sum
}

/// 指定回数以上同じペアなら発火するスコア
fn no_same_pair(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, pair_limit, shift, score): &mut (CondWrapper, isize, Shift, Score),
) -> Score {
    let mut map: HashMap<Vec<usize>, isize> = HashMap::new();
    for day in 0..schedule_prop.day_count {
        let mut i_list: Vec<usize> = Vec::new();
        for staff in 0..schedule_prop.staff_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                i_list.push(staff);
            }
        }
        if i_list.len() > 1 {
            *map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for (_, cnt) in &map {
        let a = *cnt - *pair_limit + 1;
        if a > 0 {
            ans += (a as Score) * *score
        }
    }
    ans
}
