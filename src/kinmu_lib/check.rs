//! 各段階が問題なく動いているか確認する関数のモジュール

use super::types::{
    ScheduleProp,
    ShiftState,
    Shift,
    Schedule,
};

/// すべてAbsoluteになっていないかチェック
pub fn all_absolute(hp: &ScheduleProp) -> bool {
    for r in 0..hp.staff_count {
        for c in hp.buffer..hp.day_count {
            if hp.schedule_st[r][c] != ShiftState::Absolute {
                return true;
            }
        }
    }
    return false;
}

/// IAKがすべて埋められているかチェック
pub fn safe_iak(hp: &ScheduleProp) -> bool {

    for r in 0..hp.staff_count {
        for c in 0..(hp.day_count - 1) {
            if match (hp.request[r][c], hp.request[r][c+1]) {
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
    ($w:expr, $hp: expr, $h:expr, $r:expr) => {{
        let mut cnt: isize = 0;
        for i in $hp.buffer..$hp.day_count {
            if $h[$r][i] == $w {
                cnt += 1;
            }
        }
        cnt
    }};
}

/// fillした後の表のKとIの数がちゃんとしてるかチェック
pub fn k_i_counts(hp: &ScheduleProp, h: &Schedule) -> bool {
    for r in 0..hp.staff_count {
        let k_cnt = count_waku_row!(Shift::K, hp, h, r);
        let i_cnt = count_waku_row!(Shift::I, hp, h, r);
        if hp.staff[r].k_day_count != k_cnt {
            return false;
        }
        if hp.staff[r].i_day_count != i_cnt {
            return false;
        }
    }
    return true;
}

/// Absoluteが変化していないことをチェック
pub fn abs_not_changed(hp: &ScheduleProp, h: &Schedule) -> bool {
    for r in 0..hp.staff_count {
        for c in 0..hp.day_count {
            if hp.schedule_st[r][c] == ShiftState::Absolute {
                if h[r][c] != hp.request[r][c] {
                    return false;
                }
            }
        }
    }
    return true;
}