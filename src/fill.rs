/*
fill1はUをランダムな枠に
fill2はIとKの数合わせてうまいこと埋める
*/

use crate::kata::{
    HyouProp,
    Hyou,
    WakuST,
    Waku,
};

use rand::Rng;

pub fn run(text: &str, hp: &HyouProp) -> Hyou {
    println!("{}", text);
    match text {
        "fill1" => fill_randomly1(hp),
        _ => fill_randomly1(hp),
    }
}


fn fill_randomly1(hp: &HyouProp) -> Hyou {
    let mut h = hp.kibou.clone();
    let mut rng = rand::thread_rng();
    for r in 0..hp.worker_count {
        for c in hp.buffer..hp.day_count {
            if hp.hyou_st[r][c] != WakuST::Absolute && h[r][c] == Waku::U {
                h[r][c] = [Waku::N, Waku::O, Waku::H][rng.gen_range(0..3)];
            } 
        }
    }
    h
}