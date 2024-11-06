//! 焼きなましで使う評価関数のモジュール

use super::types::{
    Schedule, ScheduleProp, Score, ScoreProp, ScoreProp::*, Shift, Shift::*, Cond, DayAttributeName, StaffAttributeName,
};

use std::collections::HashMap;

macro_rules! check_rows {
    ($check:expr, $schedule_prop: expr, $schedule:expr, $p:expr) => {{
        let mut sum = 0.0;
        for r in 0..$schedule_prop.staff_count {
            sum += $check($schedule_prop, $schedule, r, $p);
        }
        sum
    }};
}

pub fn assess_score(
    sps: &Vec<ScoreProp>,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) -> Score {
    get_score_list(sps, schedule_prop, schedule).iter().sum()
}

pub fn show_score(
    sps: &Vec<ScoreProp>,
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
    sps: &Vec<ScoreProp>,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) -> Vec<Score> {
    sps.iter()
        .map(|sp| get_score(schedule_prop, schedule, sp))
        .collect()
}

fn get_score(schedule_prop: &ScheduleProp, schedule: &Schedule, sp: &ScoreProp) -> Score {
    match sp {
        IAKpattern(p) => check_rows!(iak_pattern, schedule_prop, schedule, p),
        KIApattern(p) => check_rows!(kia_pattern, schedule_prop, schedule, p),
        KNIApattern(p) => check_rows!(knia_pattern, schedule_prop, schedule, p),
        NNIApattern(p) => check_rows!(nnia_pattern, schedule_prop, schedule, p),
        ONpattern(p) => check_rows!(on_pattern, schedule_prop, schedule, p),
        NHpattern(p) => check_rows!(nh_pattern, schedule_prop, schedule, p),
        OHpattern(p) => check_rows!(oh_pattern, schedule_prop, schedule, p),
        // WorkingDayStreak4(p) => check_rows!(working_day_streak4, schedule_prop, schedule, p),
        // WorkingDayStreak5(p) => check_rows!(working_day_streak5, schedule_prop, schedule, p),
        // WorkingDayStreak6(p) => check_rows!(working_day_streak6, schedule_prop, schedule, p),
        Streak(p) => 0.0,
        // HolidayReward(p) => check_rows!(holiday_reward, schedule_prop, schedule, p),
        Need2Holidays(p) => need_2_holidays(schedule_prop, schedule, p),
        // Need2HolidaysNoBf(p) => check_rows!(need_2_holidays_no_buffer, schedule_prop, schedule, p),
        // OHBalance(p) => check_rows!(oh_balance, schedule_prop, schedule, p),
        ShiftsBalance(p) => shifts_balance(schedule_prop, schedule, p),
        ShiftHalfBalance(p) => shift_half_balance(schedule_prop, schedule, p),
        ShiftDirPriority(p) => shift_dir_priority(schedule_prop, schedule, p),
        // KDayCount(p) => check_rows!(k_day_count, schedule_prop, schedule, p),
        // IDayCount(p) => check_rows!(i_day_count, schedule_prop, schedule, p),
        // ODayCount(p) => check_rows!(o_day_count, schedule_prop, schedule, p),
        // HDayCount(p) => check_rows!(h_day_count, schedule_prop, schedule, p),
        DayCountRegardStaffAttribute(p) => day_count_regard_staff_attribute(schedule_prop, schedule, p),
        // IStaffCount(p) => check_columns!(i_staff_count, schedule_prop, schedule, p),
        StaffCountRegardDayAttribute(p) => staff_count_regard_day_attribute(schedule_prop, schedule, p),
        // NStaffCount(p) => check_columns!(n_staff_count, schedule_prop, schedule, p),
        // OStaffCount(p) => check_columns!(o_staff_count, schedule_prop, schedule, p),
        // HStaffCount(p) => check_columns!(h_staff_count, schedule_prop, schedule, p),
        StaffCount(p) => staff_count(schedule_prop, schedule, p),
        NGPair(p) => ng_pair(schedule_prop, schedule, p),
        // LeaderAbility(p) => check_columns!(leader_ability, schedule_prop, schedule, p),
        // IAloneAbility(p) => check_columns!(i_alone_worker, schedule_prop, schedule, p),
        // IAloneBeforeBath(p) => check_columns!(i_alone_before_furo, schedule_prop, schedule, p),
        // NStaffCountWithAbility(p) => {
        //     check_columns!(n_staff_count_with_ability, schedule_prop, schedule, p)
        // }
        // NoSamePair3(p) => no_same_pair3(schedule_prop, schedule, p),
        // NoSamePair2(p) => no_same_pair2(schedule_prop, schedule, p),
        NoSamePair(p) => no_same_pair(schedule_prop, schedule, p),
        // NoUndef(p) => check_columns!(no_undef, schedule_prop, schedule, p),
    }
}

// trie木を使って連続パターンを検出したい
// まとめて実行できたら早いかも
// 木は初回実行時に構築して保持する

fn iak_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (A, K) => 0.0,
            (A, Y) => 0.0,
            (A, _) => *s,
            (I, A) => 0.0,
            (I, _) => *s,
            (_, A) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn kia_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (K, I) => *s,
            (Y, I) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn knia_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 2) {
        ans += match (schedule[r][i], schedule[r][i + 1], schedule[r][i + 2]) {
            (K, N, I) => *s,
            (K, O, I) => *s,
            (K, H, I) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn nnia_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 2) {
        ans += match (schedule[r][i], schedule[r][i + 1], schedule[r][i + 2]) {
            (N, N, I) => *s,
            (N, O, I) => *s,
            (O, O, I) => *s,
            (H, H, I) => *s,
            (H, N, I) => *s,
            _ => 0.0,
        }
    }
    -ans
}

fn on_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (O, N) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn nh_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (N, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn oh_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i + 1]) {
            (O, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn streak(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, target_shifts, streak_count, score): &(Cond, Vec<Shift>, isize, Score),
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

fn need_2_holidays(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, holidays, score): &(Cond, Vec<Shift>, Score),
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
            sum += score;
        }
    }
    sum
}

fn shifts_balance(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift1, shift2, score): &(Cond, Shift, Shift, Score),
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
        let a = d * score;
        sum += a * a;
    }
    sum
}

fn shift_half_balance(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &(Cond, Shift, Score),
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
        let a = d * score;
        sum += a * a;
    }
    sum
}

/// 指定したシフトが月の前後どちらにあるほうが良いか設定する
/// Scoreのフィールドが正なら前を優先、負なら後ろを優先
fn shift_dir_priority(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &(Cond, Shift, Score),
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
                a += score * ((i as Score) - (mid as Score));
            }
        }
        sum += a;
    }
    sum
}

fn day_count_regard_staff_attribute(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, attribute, score): &(Cond, Shift, StaffAttributeName, Score),
) -> Score {
    let mut sum = 0.0;
    for staff in 0..schedule_prop.staff_count {
        let mut cnt: isize = 0;
        for day in 0..schedule_prop.day_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                cnt += 1;
            }
        }
        let cnt_needed = schedule_prop.staff_attributes[attribute][staff];
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * score;
        sum += a * a;
    }
    sum
}

fn staff_count_regard_day_attribute(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, attribute, score): &(Cond, Shift, DayAttributeName, Score)
) -> Score {
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut cnt: isize = 0;
        for staff in 0..schedule_prop.staff_count {
            if cond.eval(staff, day, schedule_prop) && schedule[staff][day] == *shift {
                cnt += 1;
            }
        }
        let cnt_needed = schedule_prop.day_attributes[attribute][day];
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * score;
        sum += a * a;
    }
    sum
}

fn staff_count(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, count, score): &(Cond, Shift, isize, Score)
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
        let a = d * score;
        sum += a * a;
    }
    sum
}

fn ng_pair(
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    (cond, shift, score): &(Cond, Shift, Score),
) -> Score {
    // NGリストにあるペアがIかどうか確認
    let mut sum = 0.0;
    for day in 0..schedule_prop.day_count {
        let mut a = 0.0;
        for i in 0..schedule_prop.ng_list.len() {
            let (staff1, staff2) = schedule_prop.ng_list[i];
            if cond.eval(staff1, day, schedule_prop)
                && cond.eval(staff2, day, schedule_prop)
                && schedule[day][staff1] == *shift
                && schedule[day][staff2] == *shift {
                a += *score;
            }
        }
        sum += a;
    }
    sum
}

/// 指定回数以上同じペアなら発火するスコア
fn no_same_pair(schedule_prop: &ScheduleProp, schedule: &Schedule, (cond, pair_limit, shift, score): &(Cond, isize, Shift, Score)) -> Score {
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
        let a = *cnt - pair_limit + 1;
        if a > 0 {
            ans += (a as Score) * score
        }
    }
    ans
}
