//! 各段階が問題なく動いているか確認する関数のモジュール

use super::types::{Schedule, ScheduleConfig, Shift, ShiftState};

/// すべてAbsoluteになっていないかチェック
pub fn all_absolute(schedule_config: &ScheduleConfig) -> bool {
    for r in 0..schedule_config.staff.count {
        for c in schedule_config.day.buffer_count..schedule_config.day.count {
            if schedule_config.day.schedule_states[r][c] != ShiftState::Absolute {
                return true;
            }
        }
    }
    false
}

/// IAKがすべて埋められているかチェック
pub fn safe_iak(schedule_config: &ScheduleConfig) -> bool {
    for r in 0..schedule_config.staff.count {
        for c in 0..(schedule_config.day.count - 1) {
            if match (
                schedule_config.day.requested_schedule[r][c],
                schedule_config.day.requested_schedule[r][c + 1],
            ) {
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
    true
}

macro_rules! count_waku_row {
    ($w:expr, $schedule_config: expr, $schedule:expr, $r:expr) => {{
        let mut count = 0;
        for i in $schedule_config.day.buffer_count..$schedule_config.day.count {
            if $schedule[$r][i] == $w {
                count += 1;
            }
        }
        count
    }};
}

/// fillした後の表のKとIの数がちゃんとしてるかチェック
#[allow(clippy::needless_range_loop)]
pub fn k_i_counts(schedule_config: &ScheduleConfig, schedule: &Schedule) -> bool {
    for r in 0..schedule_config.staff.count {
        let k_count = count_waku_row!(Shift::K, schedule_config, schedule, r);
        let i_count = count_waku_row!(Shift::I, schedule_config, schedule, r);
        if schedule_config.get_attribute(r, &"KDayCount".to_string()) != k_count {
            return false;
        }
        if schedule_config.get_attribute(r, &"IDayCount".to_string()) != i_count {
            return false;
        }
    }
    true
}

/// Absoluteが変化していないことをチェック
#[allow(clippy::needless_range_loop)]
pub fn abs_not_changed(schedule_config: &ScheduleConfig, schedule: &Schedule) -> bool {
    for r in 0..schedule_config.staff.count {
        for c in 0..schedule_config.day.count {
            if schedule_config.day.schedule_states[r][c] == ShiftState::Absolute
                && schedule[r][c] != schedule_config.day.requested_schedule[r][c]
            {
                return false;
            }
        }
    }
    true
}
