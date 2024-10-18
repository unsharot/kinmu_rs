use crate::kata::{
    HyouProp,
    WakuST,
    Waku,
    Hyou,
};

/// すべてAbsoluteになっていないかチェック
pub fn absolute_ok(hp: &HyouProp) -> bool {
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
pub fn iak_ok(hp: &HyouProp) -> bool {

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
pub fn fill_ok(hp: &HyouProp, h: &Hyou) -> bool {
    for r in 0..hp.worker_count {
        let k_cnt = count_waku_row!(Waku::K, hp, h, r);
        let i_cnt = count_waku_row!(Waku::I, hp, h, r);
        if hp.k_counts[r] != k_cnt {
            return false;
        }
        if hp.i_counts[r] != i_cnt {
            return false;
        }
    }
    return true;
}