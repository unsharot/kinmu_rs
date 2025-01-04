//! 各段階が問題なく動いているか確認する関数のモジュール

use super::types::{Schedule, ScheduleProp, Shift, ShiftState};

/// すべてAbsoluteになっていないかチェック
pub fn all_absolute(schedule_prop: &ScheduleProp) -> bool {
    for r in 0..schedule_prop.staff_count {
        for c in schedule_prop.buffer..schedule_prop.day_count {
            if schedule_prop.schedule_st[r][c] != ShiftState::Absolute {
                return true;
            }
        }
    }
    return false;
}

/// IAKがすべて埋められているかチェック
pub fn safe_iak(schedule_prop: &ScheduleProp) -> bool {
    for r in 0..schedule_prop.staff_count {
        for c in 0..(schedule_prop.day_count - 1) {
            if match (schedule_prop.request[r][c], schedule_prop.request[r][c + 1]) {
                (Shift::A, Shift::U) => true,
                (Shift::A, _) => false,
                (Shift::I, Shift::U) => true,
                (Shift::I, _) => false,
                (Shift::U, Shift::A) => true,
                (_, Shift::A) => false,
                _ => false,
            } {
                return false;
            }
        }
    }
    return true;
}

macro_rules! count_waku_row {
    ($w:expr, $schedule_prop: expr, $schedule:expr, $r:expr) => {{
        let mut count = 0;
        for i in $schedule_prop.buffer..$schedule_prop.day_count {
            if $schedule[$r][i] == $w {
                count += 1;
            }
        }
        count
    }};
}

/// fillした後の表のKとIの数がちゃんとしてるかチェック
pub fn k_i_counts(schedule_prop: &ScheduleProp, schedule: &Schedule) -> bool {
    for r in 0..schedule_prop.staff_count {
        let k_count = count_waku_row!(Shift::K, schedule_prop, schedule, r);
        let i_count = count_waku_row!(Shift::I, schedule_prop, schedule, r);
        if schedule_prop.get_attribute(r, &"KDayCount".to_string()) != k_count {
            return false;
        }
        if schedule_prop.get_attribute(r, &"IDayCount".to_string()) != i_count {
            return false;
        }
    }
    return true;
}

/// Absoluteが変化していないことをチェック
pub fn abs_not_changed(schedule_prop: &ScheduleProp, schedule: &Schedule) -> bool {
    for r in 0..schedule_prop.staff_count {
        for c in 0..schedule_prop.day_count {
            if schedule_prop.schedule_st[r][c] == ShiftState::Absolute {
                if schedule[r][c] != schedule_prop.request[r][c] {
                    return false;
                }
            }
        }
    }
    return true;
}
