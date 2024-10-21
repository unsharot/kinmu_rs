use crate::kata::{
    HyouProp,
    WakuST,
    Waku,
    Hyou,
};

/// すべてAbsoluteになっていないかチェック
pub fn all_absolute(hp: &HyouProp) -> bool {
    for r in 0..hp.worker_count {
        for c in hp.buffer..hp.day_count {
            if hp.hyou_st[r][c] != WakuST::Absolute {
                return true;
            }
        }
    }
    return false;
}

// IAKがすべて埋められているかチェック
pub fn safe_iak(hp: &HyouProp) -> bool {

    for r in 0..hp.worker_count {
        for c in 0..(hp.day_count - 1) {
            if match (hp.kibou[r][c], hp.kibou[r][c+1]) {
                (Waku::A, Waku::U) => true,
                (Waku::A, _) => false,
                (Waku::I, Waku::U) => true,
                (Waku::I, _) => false,
                (Waku::U, Waku::A) => true,
                (_, Waku::A) => false,
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
pub fn k_i_counts(hp: &HyouProp, h: &Hyou) -> bool {
    for r in 0..hp.worker_count {
        let k_cnt = count_waku_row!(Waku::K, hp, h, r);
        let i_cnt = count_waku_row!(Waku::I, hp, h, r);
        if hp.workers[r].k_count != k_cnt {
            return false;
        }
        if hp.workers[r].i_count != i_cnt {
            return false;
        }
    }
    return true;
}

/// Absoluteが変化していないことをチェック
pub fn abs_not_changed(hp: &HyouProp, h: &Hyou) -> bool {
    for r in 0..hp.worker_count {
        for c in 0..hp.day_count {
            if hp.hyou_st[r][c] == WakuST::Absolute {
                if h[r][c] != hp.kibou[r][c] {
                    return false;
                }
            }
        }
    }
    return true;
}