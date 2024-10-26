//! 焼きなましで使う更新関数のモジュール

use super::types::{
    Waku,
    Hyou,
    WakuST,
    HyouST,
    HyouProp,
    Score,
};

use rand::Rng;


pub fn gen_update_func<'a, R: Rng>(text: &str, hp: &'a HyouProp, hst: &'a HyouST) -> Box<dyn FnMut(&Hyou, &mut R) -> Hyou + 'a> {
    println!("{}", text);
    match text {
        "update1" => Box::new(move |h, rng| update_randomly1(hp, hst, h, rng)),
        "update2" => Box::new(move |h, rng| update_randomly2(hp, hst, h, rng)),
        "update4" => Box::new(move |h, rng| update_randomly4(hp, hst, h, rng)),
        "update5" => Box::new(move |h, rng| update_randomly5(hp, hst, h, rng)),
        _ => {
            println!("MATCH SINAI UPDATE FUNC DESU!!! {}", text);
            Box::new(move |h, rng| update_randomly4(hp, hst, h, rng))
        },
    }
}

/*
元の表を更新するか、新たなものを構成するかは議論の余地あり
元の表を更新する場合、採用されなかった場合に備えて変更箇所のログを返す
変更をログをもとに戻す関数を与える必要がある

現状クローンして新たな表を返している
*/

/// ランダムな1つの枠をランダムな枠に変える
/// Absoluteの場合繰り返す
fn update_randomly1<R: Rng>(hp: &HyouProp, hst: &HyouST, h: &Hyou, rng: &mut R) -> Hyou {
    let mut newh = h.clone();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(hp.buffer..hp.day_count);
    if hst[rx][ry] != WakuST::Absolute {
        newh[rx][ry] = [Waku::N, Waku::K, Waku::I, Waku::A, Waku::O, Waku::H][rng.gen_range(0..6)];
        newh
    } else {
        update_randomly1(&hp, &hst, &h, rng)
    }
}

/// ランダムな1つの枠をランダムな枠に変える
fn update_randomly2<R: Rng>(hp: &HyouProp, hst: &HyouST, h: &Hyou, rng: &mut R) -> Hyou {
    let mut newh = h.clone();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(hp.buffer..hp.day_count);
    if hst[rx][ry] != WakuST::Absolute {
        newh[rx][ry] = [Waku::N, Waku::K, Waku::I, Waku::A, Waku::O, Waku::H][rng.gen_range(0..6)];
    }
    newh
}

/// ランダムな1つの枠をN,O,Hのうちランダムな枠に変える Absoluteなら繰り返す
fn update_randomly4<R: Rng>(hp: &HyouProp, hst: &HyouST, h: &Hyou, rng: &mut R) -> Hyou {
    let mut newh = h.clone();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(hp.buffer..hp.day_count);
    let b1 = hst[rx][ry] != WakuST::Absolute;
    let b2 = h[rx][ry] == Waku::N || h[rx][ry] == Waku::O || h[rx][ry] == Waku::H || h[rx][ry] == Waku::U;
    if b1 && b2 {
        newh[rx][ry] = [Waku::N, Waku::O, Waku::H][rng.gen_range(0..3)];
        newh
    } else {
        update_randomly4(&hp, &hst, &h, rng)
        // 合わない場合表を何個も生成することになる
        // 更新確率をpとすると、更新に必要な平均の呼び出し回数は1/p回なのでそれほど問題はない
        // むしろ何も更新せずに評価するほうが問題
    }
}

/*
各行について
1.  Iが入っていることを確認
2.  ランダムなIを取り除き、Nを代わりに置く
3.  孤立したAを取り除き、Nを代わりに置く
4.  ランダムなKを取り除き、Nを代わりに置く
5.  ランダムなNをIで置き換える
6.  Aを必要なら追加する (適当なものを置き換える あらゆる可能性あり)
7.  ランダムなNをKで置き換える
8.  K,Iの数が変わっていないことを確かめる
9.  Iの後にAが来ているか調べる
10. Absoluteが動いていないか調べる
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

fn remove_random<R: Rng>(w: Waku, hp: &HyouProp, newh: &mut Hyou, r: usize, rng: &mut R) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == w {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = Waku::N;
}

fn add_random<R: Rng>(w: Waku, hp: &HyouProp, newh: &mut Hyou, r:usize, rng: &mut R) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == Waku::N {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = w;
}

fn iak_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        ans += match (h[r][i], h[r][i+1]) {
            (Waku::A, Waku::K) => 0.0,
            (Waku::A, Waku::Y) => 0.0,
            (Waku::A, _) => *s,
            (Waku::I, Waku::A) => 0.0,
            (Waku::I, _) => *s,
            (_, Waku::A) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn remove_improper_a(hp: &HyouProp, newh: &mut Hyou, r: usize) {
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == Waku::A && newh[r][c-1] != Waku::I {
            newh[r][c] = Waku::N;
        }
    }
}

fn add_proper_a(hp: &HyouProp, newh: &mut Hyou, r: usize) {
    for c in hp.buffer..hp.day_count {
        if newh[r][c] != Waku::A && newh[r][c-1] == Waku::I {
            newh[r][c] = Waku::A;
        }
    }
}

/// IAKを破壊せずに入れ替える
/// 前提として、Absolute以外はI,A,K,Nで、AbsoluteでないO,Hはないことが条件
fn update_randomly5<R: Rng>(hp: &HyouProp, hst: &HyouST, h: &Hyou, rng: &mut R) -> Hyou {
    let mut newh = h.clone();
    for r in 0..hp.worker_count {
        // Iが入っていることを確認
        let i_cnt = count_waku_row!(Waku::I, hp, h, r);
        if i_cnt == 0 {
            // ランダムなKを取り除き、Nを代わりに置く
            remove_random(Waku::K, &hp, &mut newh, r, rng);
            // ランダムなNをKで置き換える
            add_random(Waku::K, &hp, &mut newh, r, rng);
        } else {
            // ランダムなIを取り除き、Nを代わりに置く
            remove_random(Waku::I, &hp, &mut newh, r, rng);
            // 孤立したAを取り除き、Nを代わりに置く
            remove_improper_a(&hp, &mut newh, r);
            // ランダムなKを取り除き、Nを代わりに置く
            remove_random(Waku::K, &hp, &mut newh, r, rng);
            // ランダムなNをIで置き換える
            add_random(Waku::I, &hp, &mut newh, r, rng);
            // Aを必要なら追加する (適当なものを置き換える あらゆる可能性あり)
            add_proper_a(&hp, &mut newh, r);
            // ランダムなNをKで置き換える
            add_random(Waku::K, &hp, &mut newh, r, rng);
        }

        // 条件に合うかのチェック

        // 無駄あり 一回で走査できる
        let ic1 = count_waku_row!(Waku::I, hp, h, r);
        let ic2 = count_waku_row!(Waku::I, hp, newh, r);
        let kc1 = count_waku_row!(Waku::K, hp, h, r);
        let kc2 = count_waku_row!(Waku::K, hp, newh, r);

        // Iの数に変化ないか
        let b1 = ic1 == ic2;
        
        // Kの数に変化ないか
        let b2 = kc1 == kc2;
        
        // IAKの連続が崩れていないか
        let b3 = iak_renzoku(hp, h, r, &1000.0) >= iak_renzoku(hp, &newh, r, &1000.0);
        
        // Absoluteが変化していないか
        let b4 = {
            let mut ans = true;
            for c in hp.buffer..hp.day_count {
                if hst[r][c] == WakuST::Absolute {
                    ans = ans && h[r][c] == newh[r][c];
                }
            }
            ans
        };

        // もし変化が不適切なら
        if ! (b1 && b2 && b3 && b4) {
            // 戻す
            for c in hp.buffer..hp.day_count {
                newh[r][c] = h[r][c];
            }
        }
    }
    newh
}