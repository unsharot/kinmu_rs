//! 焼きなましで使う評価関数のモジュール

use super::types::{
    CondWrapper, DayAttributeName, Schedule, ScheduleConfig, Score, ScoreProp, Shift,
    StaffAttributeName,
};

use std::collections::HashMap;

pub fn assess_score(
    sps: &mut Vec<ScoreProp>,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) -> Score {
    get_score_list(sps, schedule_config, schedule).iter().sum()
}

#[allow(clippy::ptr_arg)]
fn get_score_list(
    sps: &mut Vec<ScoreProp>,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) -> Vec<Score> {
    sps.iter_mut()
        .map(|sp: &mut ScoreProp| get_score(schedule_config, schedule, sp))
        .collect()
}

fn get_score(schedule_config: &ScheduleConfig, schedule: &Schedule, sp: &mut ScoreProp) -> Score {
    match sp {
        ScoreProp::PatternGeneral(p) => pattern_general(schedule_config, schedule, p),
        ScoreProp::PatternFixed(p) => pattern_fixed(schedule_config, schedule, p),
        ScoreProp::PatternGeneralAny(p) => pattern_general_any(schedule_config, schedule, p),
        ScoreProp::PatternFixedAny(p) => pattern_fixed_any(schedule_config, schedule, p),
        ScoreProp::Streak(p) => streak(schedule_config, schedule, p),
        ScoreProp::ShiftsBalance(p) => shifts_balance(schedule_config, schedule, p),
        ScoreProp::ShiftHalfBalance(p) => shift_half_balance(schedule_config, schedule, p),
        ScoreProp::ShiftDirPriority(p) => shift_dir_priority(schedule_config, schedule, p),
        ScoreProp::DayCountRegardStaffAttribute(p) => {
            day_count_regard_staff_attribute(schedule_config, schedule, p)
        }
        ScoreProp::StaffCountRegardDayAttribute(p) => {
            staff_count_regard_day_attribute(schedule_config, schedule, p)
        }
        ScoreProp::StaffCount(p) => staff_count(schedule_config, schedule, p),
        ScoreProp::StaffCountAtLeast(p) => staff_count_at_least(schedule_config, schedule, p),
        ScoreProp::StaffCountWithPremise(p) => {
            staff_count_with_premise(schedule_config, schedule, p)
        }
        ScoreProp::NGPair(p) => ng_pair(schedule_config, schedule, p),
        ScoreProp::NoSamePair(p) => no_same_pair(schedule_config, schedule, p),
    }
}

/// 指定したシフトパターンの数に応じて発火するスコア
/// ただし、シフトパターンは複数候補を指定可能
/// TODO: HashMapやTrie木を用いた高速化
#[allow(clippy::needless_range_loop)]
fn pattern_general(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Vec<Shift>>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut a = 0.0;
        for day in 0..schedule_config.day.count {
            let mut hit = true;
            let mut is_valid = false;
            for dd in 0..shift_pattern.len() {
                if schedule_config.day.count <= day + dd {
                    hit = false;
                    break;
                } else if cond.eval(staff, day + dd, schedule_config) {
                    is_valid = true;
                    if !(shift_pattern[dd].contains(&schedule[staff][day + dd])) {
                        hit = false;
                        break;
                    }
                } else {
                    hit = false;
                    break;
                }
            }
            if hit && is_valid {
                a += *score;
                break;
            }
        }
        sum += a;
    }
    sum
}

/// 指定したシフトパターンの数に応じて発火するスコア
/// TODO: HashMapやTrie木を用いた高速化
#[allow(clippy::needless_range_loop)]
fn pattern_fixed(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut a = 0.0;
        for day in 0..schedule_config.day.count {
            let mut hit = true;
            let mut is_valid = false;
            for dd in 0..shift_pattern.len() {
                if schedule_config.day.count <= day + dd {
                    hit = false;
                    break;
                } else if cond.eval(staff, day + dd, schedule_config) {
                    is_valid = true;
                    if shift_pattern[dd] != schedule[staff][day + dd] {
                        hit = false;
                        break;
                    }
                } else {
                    hit = false;
                    break;
                }
            }
            if hit && is_valid {
                a += *score;
                break;
            }
        }
        sum += a;
    }
    sum
}

/// 指定したパターンが存在するスタッフに対して発火するスコア
/// TODO: HashMapやTrie木を用いた高速化
#[allow(clippy::needless_range_loop)]
fn pattern_general_any(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Vec<Shift>>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut any = false;
        for day in 0..schedule_config.day.count {
            let mut hit = true;
            let mut is_valid = false;
            for dd in 0..shift_pattern.len() {
                if schedule_config.day.count <= day + dd {
                    hit = false;
                    break;
                } else if cond.eval(staff, day + dd, schedule_config) {
                    is_valid = true;
                    if !(shift_pattern[dd].contains(&schedule[staff][day + dd])) {
                        hit = false;
                        break;
                    }
                } else {
                    hit = false;
                    break;
                }
            }
            if hit && is_valid {
                any = true;
                break;
            }
        }
        if any {
            sum += *score;
        }
    }
    sum
}

/// 指定したパターンが存在するスタッフに対して発火するスコア
/// ただし、パターンは固定
/// TODO: HashMapやTrie木を用いた高速化
#[allow(clippy::needless_range_loop)]
fn pattern_fixed_any(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift_pattern, score): &mut (CondWrapper, Vec<Shift>, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut any = false;
        for day in 0..schedule_config.day.count {
            let mut hit = true;
            let mut is_valid = false;
            for dd in 0..shift_pattern.len() {
                if schedule_config.day.count <= day + dd {
                    hit = false;
                    break;
                } else if cond.eval(staff, day + dd, schedule_config) {
                    is_valid = true;
                    if shift_pattern[dd] != schedule[staff][day + dd] {
                        hit = false;
                        break;
                    }
                } else {
                    hit = false;
                    break;
                }
            }
            if hit && is_valid {
                any = true;
                break;
            }
        }
        if any {
            sum += *score;
        }
    }
    sum
}

/// 指定したシフトが指定回数連続して存在するか判定するスコア
/// 指定回数+1回連続は1回分としてカウントされる
#[allow(clippy::needless_range_loop)]
fn streak(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, target_shifts, streak_count, score): &mut (CondWrapper, Vec<Shift>, i32, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut a = 0.0;
        let mut accum = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
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

/// 指定した2つのシフト数がスタッフあたりでバランス良いか判定するスコア
#[allow(clippy::needless_range_loop)]
fn shifts_balance(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &mut (CondWrapper, Shift, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut count1: i32 = 0;
        let mut count2: i32 = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift1 {
                    count1 += 1;
                }
                if schedule[staff][day] == *shift2 {
                    count2 += 1;
                }
            }
        }
        if is_valid {
            let d = (count1 - count2).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

/// 指定したシフトが月の前後でバランスよく配置されているかを判定するスコア
#[allow(clippy::needless_range_loop)]
fn shift_half_balance(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut is_valid = false;
        // 条件を満たすdayの中から中間を探す
        let mut len = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    len += 1;
                }
            }
        }
        let mid = len / 2;

        let mut cf: i32 = 0;
        let mut cl: i32 = 0;
        let mut i = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    i += 1;
                    if i < mid {
                        cf += 1;
                    } else {
                        cl += 1;
                    }
                }
            }
        }
        if is_valid {
            let d = (cf - cl).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

/// 指定したシフトが月の前後どちらにあるほうが良いか設定する
/// Scoreのフィールドが正なら前を優先、負なら後ろを優先
#[allow(clippy::needless_range_loop)]
fn shift_dir_priority(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut is_valid = false;
        // 条件を満たすdayの中から中間を探す
        let mut len = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    len += 1;
                }
            }
        }
        let mid = len / 2;

        let mut a = 0.0;
        let mut i = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    i += 1;
                    a += *score * ((i as Score) - (mid as Score));
                }
            }
        }
        if is_valid {
            sum += a;
        }
    }
    sum
}

/// 指定したシフトをStaffAttributeで指定した数入らなかった場合に発火するスコア
#[allow(clippy::needless_range_loop)]
fn day_count_regard_staff_attribute(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, StaffAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_config.staff.count {
        let mut is_valid = false;
        let mut count = 0;
        for day in 0..schedule_config.day.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    count += 1;
                }
            }
        }
        if is_valid {
            let count_needed = schedule_config.get_attribute(staff, attribute);
            if count_needed != -1 {
                // 値が-1の場合、任意の数を許すためスコアを増やさない
                let d = (count - count_needed).abs() as Score;
                let a = d * *score;
                sum += a * a;
            }
        }
    }
    sum
}

/// 指定したシフトがDayAttributeで指定した数いない場合に発火するスコア
#[allow(clippy::needless_range_loop)]
fn staff_count_regard_day_attribute(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, attribute, score): &mut (CondWrapper, Shift, DayAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    count += 1;
                }
            }
        }
        if is_valid {
            let count_needed = schedule_config.day.attributes.get(attribute).unwrap()[day];
            let d = (count - count_needed).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

/// 指定したシフトが指定した数いない場合に発火するスコア
#[allow(clippy::needless_range_loop)]
fn staff_count(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, count, score): &mut (CondWrapper, Shift, i32, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut staff_count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    staff_count += 1;
                }
            }
        }
        if is_valid {
            let d = (staff_count - *count).abs() as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

/// 指定したシフトが指定した数より少ない場合に発火するスコア
#[allow(clippy::needless_range_loop)]
fn staff_count_at_least(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, count, score): &mut (CondWrapper, Shift, i32, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut is_valid = false;
        let mut staff_count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) {
                is_valid = true;
                if schedule[staff][day] == *shift {
                    staff_count += 1;
                }
            }
        }
        if is_valid {
            let d = std::cmp::min(staff_count - *count, 0) as Score;
            let a = d * *score;
            sum += a * a;
        }
    }
    sum
}

#[allow(clippy::needless_range_loop)]
fn staff_count_with_premise(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond_premise, shift_premise, count_premise, cond_main, shift_main, count_main, score): &mut (
        CondWrapper,
        Shift,
        i32,
        CondWrapper,
        Shift,
        i32,
        Score,
    ),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut count = 0;
        for staff in 0..schedule_config.staff.count {
            if cond_premise.eval(staff, day, schedule_config)
                && schedule[staff][day] == *shift_premise
            {
                count += 1;
            }
        }
        if count == *count_premise {
            let mut is_valid = false;
            let mut count2 = 0;
            for staff in 0..schedule_config.staff.count {
                if cond_main.eval(staff, day, schedule_config) {
                    is_valid = true;
                    if schedule[staff][day] == *shift_main {
                        count2 += 1;
                    }
                }
            }
            if is_valid {
                let d = (count2 - *count_main).abs() as Score;
                let a = d * *score;
                sum += a * a;
            }
        }
    }
    sum
}

/// NGリストにあるペアがともに指定したシフトなら発火するスコア
fn ng_pair(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, shift, score): &mut (CondWrapper, Shift, Score),
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_config.day.count {
        let mut a = 0.0;
        for i in 0..schedule_config.staff.ng_list.len() {
            let (staff1, staff2) = schedule_config.staff.ng_list[i];
            if cond.eval(staff1, day, schedule_config)
                && cond.eval(staff2, day, schedule_config)
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
#[allow(clippy::needless_range_loop)]
fn no_same_pair(
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    (cond, pair_limit, shift, score): &mut (CondWrapper, i32, Shift, Score),
) -> Score {
    let mut pair_map: HashMap<Vec<usize>, i32> = HashMap::new();
    for day in 0..schedule_config.day.count {
        let mut i_list: Vec<usize> = Vec::new();
        for staff in 0..schedule_config.staff.count {
            if cond.eval(staff, day, schedule_config) && schedule[staff][day] == *shift {
                i_list.push(staff);
            }
        }
        // ある日の夜勤の人数が2人以上ならペアのマップに加算
        if i_list.len() >= 2 {
            *pair_map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for count in pair_map.values() {
        let a = *count - *pair_limit + 1;
        if a > 0 {
            ans += (a as Score) * *score
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use crate::kinmu_lib::types::Cond;

    use super::*;

    #[test]
    fn general_pattern_test() {
        let mut sc: ScheduleConfig = Default::default();
        sc.staff.count = 1;
        sc.day.count = 4;

        let score = pattern_general(
            &sc,
            &vec![vec![Shift::H, Shift::H, Shift::A, Shift::Y]],
            &mut (
                CondWrapper::new(Cond::Every),
                vec![
                    vec![Shift::N, Shift::O, Shift::H, Shift::A, Shift::K, Shift::Y],
                    vec![Shift::A],
                ],
                1.0,
            ),
        );
        assert_eq!(1.0, score);
    }

    #[test]
    fn general_pattern_any_with_cond() {
        let mut sc: ScheduleConfig = Default::default();
        sc.staff.count = 1;
        sc.day.count = 37;
        sc.day.buffer_count = 3;
        let schedule = {
            use Shift::*;
            &vec![vec![
                N, K, K, K, O, I, A, K, H, O, K, H, N, I, A, K, H, I, A, K, O, N, I, A, K, N, O, N,
                K, I, A, K, H, I, A, K, O,
            ]]
        };

        let score = pattern_general_any(
            &sc,
            &schedule,
            &mut (
                CondWrapper::new(Cond::DayExceptBuffer),
                vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
                -1000.0,
            ),
        );
        assert_eq!(0.0, score);

        let score = pattern_general_any(
            &sc,
            &schedule,
            &mut (
                CondWrapper::new(Cond::Every),
                vec![vec![Shift::K, Shift::Y], vec![Shift::K, Shift::Y]],
                -1000.0,
            ),
        );
        assert_eq!(-1000.0, score);
    }
}
