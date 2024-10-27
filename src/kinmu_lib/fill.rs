//! 初めに勤務表を埋める関数のモジュール

/*
fill1はUをランダムな枠に
fill2はIとKの数合わせてうまいこと埋める
*/

use super::types::{
    ScheduleProp,
    Schedule,
    ShiftState,
    Shift,
    FillConfig,
};
use crate::seed;

use rand::Rng;

pub fn run(fc: &FillConfig, hp: &ScheduleProp) -> Schedule {
    println!("{}", fc.name);
    let mut rng = seed::gen_rng_from_seed(fc.seed);
    match fc.name.as_str() {
        "fill1" => fill_randomly1(hp, &mut rng),
        "fill2" => fill_randomly2(hp, &mut rng),
        _ => {
            println!("MATCH SINAI FILL FUNC DESU!!! {}", fc.name);
            fill_randomly1(hp, &mut rng)
        },
    }
}


fn fill_randomly1<R: Rng>(hp: &ScheduleProp, rng: &mut R) -> Schedule {
    let mut h = hp.request.clone();
    for r in 0..hp.staff_count {
        for c in hp.buffer..hp.day_count {
            if hp.schedule_st[r][c] != ShiftState::Absolute && h[r][c] == Shift::U {
                h[r][c] = [Shift::N, Shift::O, Shift::H][rng.gen_range(0..3)];
            } 
        }
    }
    h
}

/*
1.  連続するRandomの個数をリストアップ
2.  それぞれ3で割って商と余りを出す
3.  商だけIAKを、余りだけNを並べる
4.  Iの差分を計算
5.  余分なIをランダムに消す
6.  孤立したAを消す
7.  Kの差分を計算
8.  不足したKをランダムに足す
9.  余分なKを消した中で、最もIAKrenzokuが低いものを採用
10. 元の表に埋め込む

もとの表に埋め込んでから余分なKを消すほうがいい
*/

/*
Kが足りない場合でも、続行できるか
*/

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

fn remove_random<R: Rng>(w: Shift, hp: &ScheduleProp, newh: &mut Schedule, r: usize, rng: &mut R) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == w && hp.schedule_st[r][c] != ShiftState::Absolute {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = Shift::N;
}

fn remove_improper_a(hp: &ScheduleProp, newh: &mut Schedule, r: usize) {
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == Shift::A && newh[r][c-1] != Shift::I && hp.schedule_st[r][c] != ShiftState::Absolute {
            newh[r][c] = Shift::N;
        }
    }
}

fn add_random<R: Rng>(w: Shift, hp: &ScheduleProp, newh: &mut Schedule, r:usize, rng: &mut R) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        // if newh[r][c] == Shift::N || newh[r][c] == Shift::U {
        if newh[r][c] == Shift::N && hp.schedule_st[r][c] != ShiftState::Absolute {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = w;
}

fn fill_randomly2<R: Rng>(hp: &ScheduleProp, rng: &mut R) -> Schedule {
    let mut h = hp.request.clone();
    for r in 0..hp.staff_count {

        // 直接IAKN構築する
        let mut r_cnt = 0;
        for c in hp.buffer..(hp.day_count + 1) {
            // Randomが途切れることを検知して、途切れるなら入るだけIAKを入れる
            // なお、最後は途切れないとしてIAKが埋まるだけ埋める
            if c != hp.day_count && hp.schedule_st[r][c] == ShiftState::Random {
                r_cnt += 1;
                if r_cnt == 3 {
                    r_cnt = 0;
                    h[r][c-2] = Shift::I;
                    h[r][c-1] = Shift::A;
                    h[r][c] = Shift::K;
                }
            } else if c == hp.day_count {
                if r_cnt == 1 {
                    h[r][c-1] = Shift::I;
                } else if r_cnt == 2 {
                    h[r][c-2] = Shift::I;
                    h[r][c-1] = Shift::A;
                }
            } else {
                if r_cnt == 1 {
                    h[r][c-1] = Shift::N;
                } else if r_cnt == 2 {
                    if h[r][c] == Shift::K || h[r][c] == Shift::Y {
                        h[r][c-2] = Shift::I;
                        h[r][c-1] = Shift::A;
                    } else {
                        h[r][c-2] = Shift::N;
                        h[r][c-1] = Shift::N;
                    }
                }
                r_cnt = 0;
            }
        }

        // Iの差分を計算
        let i_dif = count_waku_row!(Shift::I, hp, h, r) - hp.staff[r].i_day_count;

        // 余分なIをランダムに消す
        for _ in 0..i_dif {
            remove_random(Shift::I, &hp, &mut h, r, rng);
        }

        // 孤立したAを消す
        remove_improper_a(&hp, &mut h, r);

        // Kの差分を計算
        let k_dif = hp.staff[r].k_day_count - count_waku_row!(Shift::K, hp, h, r);

        if k_dif > 0 {
            // 不足したKをランダムに足す
            for _ in 0..k_dif {
                add_random(Shift::K, &hp, &mut h, r, rng);
            }
        } else {
            // 余分なKを消した中で、最もIAKrenzokuが低いものを採用

            // 孤立KとそうでないKのインデックスをとる
            let mut k_nc_ids = Vec::new();
            let mut k_ng_ids = Vec::new();
            for c in hp.buffer..hp.day_count {
                if (h[r][c] == Shift::K) && (hp.schedule_st[r][c] == ShiftState::Random) {
                    if h[r][c-1] == Shift::A {
                        k_ng_ids.push(c);
                    } else {
                        k_nc_ids.push(c);
                    }
                }
            }

            // Kを消す
            for _ in 0..-k_dif {
                if k_nc_ids.is_empty() {
                    let rnd = rng.gen_range(0..k_ng_ids.len());
                    h[r][k_ng_ids[rnd]] = Shift::N;
                    k_ng_ids.remove(rnd);
                } else {
                    let rnd = rng.gen_range(0..k_nc_ids.len());
                    h[r][k_nc_ids[rnd]] = Shift::N;
                    k_nc_ids.remove(rnd);
                }
            }
            // TODO: k_ng_idsが空になった場合のエラーハンドリング
        }
    }
    h
}