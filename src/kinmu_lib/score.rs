//! 焼きなましで使う評価関数のモジュール

use super::types::{
    Shift,
    Shift::*,
    Schedule,
    Score,
    DayState,
    ScoreProp,
    ScoreProp::*,
    ScheduleProp,
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

macro_rules! check_columns {
    ($check:expr, $schedule_prop: expr, $schedule:expr, $p:expr) => {{
        let mut sum = 0.0;
        for c in $schedule_prop.buffer..$schedule_prop.day_count {
            sum += $check($schedule_prop, $schedule, c, $p);
        }
        sum
    }};
}

pub fn assess_score(sps: &Vec<ScoreProp>, schedule_prop: &ScheduleProp, schedule: &Schedule) -> Score {
    get_score_list(sps, schedule_prop, schedule).iter().sum()
}

pub fn show_score(sps: &Vec<ScoreProp>, schedule_prop: &ScheduleProp, schedule: &Schedule) -> String {
    let sl = get_score_list(sps, schedule_prop, schedule);
    let ss: Vec<String> = sps.iter().map(|x| x.to_string()).collect();
    let zipped: Vec<String> = ss.iter().zip(sl.iter()).map(|(x,y)| x.to_string() + " : " + &y.to_string()).collect();
    zipped.join("\n")
}

fn get_score_list(sps: &Vec<ScoreProp>, schedule_prop: &ScheduleProp, schedule: &Schedule) -> Vec<Score> {
    sps.iter().map(|sp| get_score(schedule_prop, schedule, sp)).collect()
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
        WorkingDayStreak4(p) => check_rows!(working_day_streak4, schedule_prop, schedule, p),
        WorkingDayStreak5(p) => check_rows!(working_day_streak5, schedule_prop, schedule, p),
        WorkingDayStreak6(p) => check_rows!(working_day_streak6, schedule_prop, schedule, p),
        HolidayReward(p) => check_rows!(holiday_reward, schedule_prop, schedule, p),
        Need2Holidays(p) => check_rows!(need_2_holidays, schedule_prop, schedule, p),
        Need2HolidaysNoBf(p) => check_rows!(need_2_holidays_no_buffer, schedule_prop, schedule, p),
        OHBalance(p) => check_rows!(oh_balance, schedule_prop, schedule, p),
        ShiftHalfBalance(p) => check_rows!(shift_half_balance, schedule_prop, schedule, p),
        ShiftDirPriority(p) => check_rows!(shift_dir_priority, schedule_prop, schedule, p),
        KDayCount(p) => check_rows!(k_day_count, schedule_prop, schedule, p),
        IDayCount(p) => check_rows!(i_day_count, schedule_prop, schedule, p),
        ODayCount(p) => check_rows!(o_day_count, schedule_prop, schedule, p),
        HDayCount(p) => check_rows!(h_day_count, schedule_prop, schedule, p),
        IStaffCount(p) => check_columns!(i_staff_count, schedule_prop, schedule, p),
        NStaffCount(p) => check_columns!(n_staff_count, schedule_prop, schedule, p),
        OStaffCount(p) => check_columns!(o_staff_count, schedule_prop, schedule, p),
        HStaffCount(p) => check_columns!(h_staff_count, schedule_prop, schedule, p),
        NGPair(p) => check_columns!(ng_pair, schedule_prop, schedule, p),
        LeaderAbility(p) => check_columns!(leader_ability, schedule_prop, schedule, p),
        IAloneAbility(p) => check_columns!(i_alone_worker, schedule_prop, schedule, p),
        IAloneBeforeBath(p) => check_columns!(i_alone_before_furo, schedule_prop, schedule, p),
        NStaffCountWithAbility(p) => check_columns!(n_staff_count_with_ability, schedule_prop, schedule, p),
        NoSamePair3(p) => no_same_pair3(schedule_prop, schedule, p),
        NoSamePair2(p) => no_same_pair2(schedule_prop, schedule, p),
        NoUndef(p) => check_columns!(no_undef, schedule_prop, schedule, p),
    }
}













// trie木を使って連続パターンを検出したい
// まとめて実行できたら早いかも
// 木は初回実行時に構築して保持する

fn iak_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i+1]) {
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
        ans += match (schedule[r][i], schedule[r][i+1]) {
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
        ans += match (schedule[r][i], schedule[r][i+1], schedule[r][i+2]) {
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
        ans += match (schedule[r][i], schedule[r][i+1], schedule[r][i+2]) {
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
        ans += match (schedule[r][i], schedule[r][i+1]) {
            (O, N) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn nh_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i+1]) {
            (N, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn oh_pattern(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i+1]) {
            (O, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn working_day_streak4(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let working_shifts = [N, O, H, I];
    for i in 0..(schedule_prop.day_count - 3) {
        if working_shifts.contains(&schedule[r][i]) && working_shifts.contains(&schedule[r][i+1]) && working_shifts.contains(&schedule[r][i+2]) && working_shifts.contains(&schedule[r][i+3]) {
            if schedule[r][i+3] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

fn working_day_streak5(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let working_shifts = [N, O, H, I];
    for i in 0..(schedule_prop.day_count - 4) {
        if working_shifts.contains(&schedule[r][i]) && working_shifts.contains(&schedule[r][i+1]) && working_shifts.contains(&schedule[r][i+2]) && working_shifts.contains(&schedule[r][i+3]) && working_shifts.contains(&schedule[r][i+4]) {
            if schedule[r][i+4] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

fn working_day_streak6(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let working_shifts = [N, O, H, I];
    for i in 0..(schedule_prop.day_count - 5) {
        if working_shifts.contains(&schedule[r][i]) && working_shifts.contains(&schedule[r][i+1]) && working_shifts.contains(&schedule[r][i+2]) && working_shifts.contains(&schedule[r][i+3]) && working_shifts.contains(&schedule[r][i+4]) && working_shifts.contains(&schedule[r][i+5]) {
            if schedule[r][i+5] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

fn holiday_reward(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(schedule_prop.day_count - 1) {
        ans += match (schedule[r][i], schedule[r][i+1]) {
            (K, K) => *s,
            (K, Y) => *s,
            (Y, K) => *s,
            (Y, Y) => *s,
            _ => 0.0,
        }
    }
    -ans
}

fn need_2_holidays(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut check = false;
    for i in 0..(schedule_prop.day_count - 1) {
        check = check || match (schedule[r][i], schedule[r][i+1]) {
            (K, K) => true,
            (K, Y) => true,
            (Y, K) => true,
            (Y, Y) => true,
            _ => false,
        }
    }
    if check {
        0.0
    } else {
        *s
    }
}

fn need_2_holidays_no_buffer(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut check = false;
    for i in schedule_prop.buffer..(schedule_prop.day_count - 1) {
        check = check || match (schedule[r][i], schedule[r][i+1]) {
            (K, K) => true,
            (K, Y) => true,
            (Y, K) => true,
            (Y, Y) => true,
            _ => false,
        }
    }
    if check {
        0.0
    } else {
        *s
    }
}

fn oh_balance(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let mut o: isize = 0;
    let mut h: isize = 0;
    for i in schedule_prop.buffer..schedule_prop.day_count {
        if schedule[r][i] == O {
            o += 1;
        } else if schedule[r][i] == H {
            h += 1;
        }
    }
    let d = (h - o).abs() as Score;
    let a = d * s;
    a * a
}

fn shift_half_balance(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, (shift, s): &(Shift, Score)) -> Score {
    let mut cf: isize = 0;
    let mut cl: isize = 0;
    for i in schedule_prop.buffer..((schedule_prop.day_count - schedule_prop.buffer) / 2) {
        if schedule[r][i] == *shift {
            cf += 1;
        }
    }
    for i in ((schedule_prop.day_count - schedule_prop.buffer) / 2)..schedule_prop.day_count {
        if schedule[r][i] == *shift {
            cl += 1;
        }
    }
    let d = (cf - cl).abs() as Score;
    let a = d * s;
    a * a
}

/// 指定したシフトが月の前後どちらにあるほうが良いか設定する
/// Scoreのフィールドが正なら前を優先、負なら後ろを優先
fn shift_dir_priority(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, (shift, s): &(Shift, Score)) -> Score {
    let mut ans: Score = 0.0;
    let mid = schedule_prop.buffer + ((schedule_prop.day_count - schedule_prop.buffer) / 2);
    for i in schedule_prop.buffer..schedule_prop.day_count {
        if schedule[r][i] == *shift {
            ans += s * ((i as Score) - (mid as Score));
        }
    }
    ans
}

macro_rules! count_waku_row {
    ($shift:expr, $schedule_prop: expr, $schedule:expr, $r:expr) => {{
        let mut cnt: isize = 0;
        for i in $schedule_prop.buffer..$schedule_prop.day_count {
            if $schedule[$r][i] == $shift {
                cnt += 1;
            }
        }
        cnt
    }};
}

fn k_day_count(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let cnt_needed = schedule_prop.staff_list[r].k_day_count;
    let cnt = count_waku_row!(K, schedule_prop, schedule, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn i_day_count(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let cnt_needed = schedule_prop.staff_list[r].i_day_count;
    let cnt = count_waku_row!(I, schedule_prop, schedule, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn o_day_count(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let cnt_needed = schedule_prop.staff_list[r].o_day_count;
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(O, schedule_prop, schedule, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    }
}

fn h_day_count(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize, s: &Score) -> Score {
    let cnt_needed = schedule_prop.staff_list[r].h_day_count;
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(H, schedule_prop, schedule, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    }
}













macro_rules! count_waku_column {
    ($shift:expr, $schedule_prop: expr, $schedule:expr, $c:expr) => {{
        let mut cnt: isize = 0;
        for i in 0..$schedule_prop.staff_count {
            if $schedule[i][$c] == $shift {
                cnt += 1;
            }
        }
        cnt
    }};
}

fn i_staff_count(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, s: &Score) -> Score {
    let cnt_needed = schedule_prop.i_staff_count[c - schedule_prop.buffer];
    let cnt = count_waku_column!(I, schedule_prop, schedule, c);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn n_staff_count(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (d,cnt_needed,s): &(DayState,isize,Score)) -> Score {
    if schedule_prop.days[c] == *d {
        let cnt = count_waku_column!(N, schedule_prop, schedule, c);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    } else {
        0.0
    }
}

fn o_staff_count(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (cnt_needed, s): &(isize,Score)) -> Score {
    let cnt = count_waku_column!(O, schedule_prop, schedule, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn h_staff_count(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (cnt_needed, s): &(isize,Score)) -> Score {
    let cnt = count_waku_column!(H, schedule_prop, schedule, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn ng_pair(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, s: &Score) -> Score {
    // NGリストにあるペアがIかどうか確認
    let mut ans = 0.0;
    for i in 0..schedule_prop.ng_list.len() {
        let (a, b) = schedule_prop.ng_list[i];
        if (schedule[a-1][c] == I && schedule[b-1][c] == I) || (schedule[a-1][c] == A && schedule[b-1][c] == A) {
            ans += s;
        }
    }
    ans
}

fn leader_ability(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (ab, s): &(isize,Score)) -> Score {
    if matches!(schedule_prop.days[c], DayState::Holiday) {
        let mut a_cnt = 0;
        for r in 0..schedule_prop.staff_count {
            if (schedule[r][c] == N) && ((schedule_prop.staff_list[r].ability % ab) != 0) {
                    a_cnt += 1;
            }
        }
        if a_cnt == 0 {
            *s
        } else {
            0.0
        }
    } else {
        0.0
    }
}

///一人で夜勤が可能か
fn i_alone_worker(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (ab, s): &(isize,Score)) -> Score {
    let mut i_cnt = 0;
    let mut a_cnt = 0;
    for r in 0..schedule_prop.staff_count {
        if schedule[r][c] == I {
            i_cnt += 1;
            if (schedule_prop.staff_list[r].ability % ab) != 0 {
                a_cnt += 1;
            }
        }
    }
    if (i_cnt == 1) && (a_cnt == 0) {
        *s
    } else {
        0.0
    }
}

fn i_alone_before_furo(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, s: &Score) -> Score {
    if schedule_prop.days[c - 1] == DayState::Bath {
        let mut i_cnt = 0;
        for r in 0..schedule_prop.staff_count {
            if schedule[r][c] == I {
                    i_cnt += 1;
            }
        }
        if i_cnt <= 1 {
            *s
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// 能力条件を満たすスタッフが指定した人数いない場合のペナルティを設定
/// 部屋持ちとO,Hできる人をIAKで保持するために使用
fn n_staff_count_with_ability(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, (cnt_needed, ab, s): &(isize,isize,Score)) -> Score {
    let mut a_cnt = 0;
    for r in 0..schedule_prop.staff_count {
        if (schedule[r][c] == N) && ((schedule_prop.staff_list[r].ability % ab) != 0) {
                a_cnt += 1;
        }
    }
    let d = if *cnt_needed > a_cnt {
        (*cnt_needed - a_cnt) as Score
    } else {
        0.0
    };
    s * d * d
}

/// 3回以上同じペアなら発火するスコア
fn no_same_pair3(schedule_prop: &ScheduleProp, schedule: &Schedule, s: &Score) -> Score {
    let mut map: HashMap<Vec<usize>, isize> = HashMap::new();
    for c in schedule_prop.buffer..schedule_prop.day_count {
        let mut i_list: Vec<usize> = Vec::new();
        for r in 0..schedule_prop.staff_count {
            if matches!(schedule[r][c], I) {
                i_list.push(r)
            }
        }
        if i_list.len() > 1 {
            *map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for (_, cnt) in &map {
        let a = *cnt - 2;
        if a > 0 {
            ans += (a as Score) * s
        }
    }
    ans
}

/// 2回以上同じペアなら発火するスコア
fn no_same_pair2(schedule_prop: &ScheduleProp, schedule: &Schedule, s: &Score) -> Score {
    let mut map: HashMap<Vec<usize>, isize> = HashMap::new();
    for c in schedule_prop.buffer..schedule_prop.day_count {
        let mut i_list: Vec<usize> = Vec::new();
        for r in 0..schedule_prop.staff_count {
            if matches!(schedule[r][c], I) {
                i_list.push(r)
            }
        }
        if i_list.len() > 1 {
            *map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for (_, cnt) in &map {
        let a = *cnt - 1;
        if a > 0 {
            ans += (a as Score) * s
        }
    }
    ans
}

fn no_undef(schedule_prop: &ScheduleProp, schedule: &Schedule, c: usize, s: &Score) -> Score {
    let cnt = count_waku_column!(U, schedule_prop, schedule, c);
    let d = cnt as Score;
    let a = d * s;
    a * a
}