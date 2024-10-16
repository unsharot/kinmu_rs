use crate::kata::{
    Waku,
    Hyou,
    WakuST,
    HyouST,
    HyouProp,
};

use rand::Rng;
use rand::rngs::ThreadRng;


pub fn gen_update_func<'a>(text: &str, hp: &'a HyouProp, hst: &'a HyouST) -> Box<dyn FnMut(&Hyou) -> Hyou + 'a> {
    println!("{}", text);
    match text {
        "update1" => Box::new(move |h| update_randomly1(hp, hst, h)),
        "update2" => Box::new(move |h| update_randomly2(hp, hst, h)),
        "update4" => Box::new(move |h| update_randomly4(hp, hst, h)),
        "update5" => Box::new(move |h| update_randomly5(hp, hst, h)),
        _ => Box::new(move |h| update_randomly4(hp, hst, h)),
    }
}

//元の表を更新するか、新たなものを構成するかは考える必要がある
//とりあえず元の表を更新する方針で
//annealing.rsは新たなモデルを渡す方針なのでどちらか変える必要あり
//元の表を更新する方針だと採用されなかった場合に戻せないかも
//変更箇所のログを返す必要がある
//変更をログをもとに戻す関数を与える必要がある
// pub fn update_randomly(hst: &HyouST, h: &mut Hyou) {

// }

//上の方法でなければこれ
// pub fn update_randomly(hst: &HyouST, h: &Hyou) -> Hyou {

// }

/// ランダムな1つの枠をランダムな枠に変えるAbsoluteの場合繰り返す
fn update_randomly1(hp: &HyouProp, hst: &HyouST, h: &Hyou) -> Hyou {
    let mut newh = h.clone();
    let mut rng = rand::thread_rng();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(0..hp.day_count);
    if hst[rx][ry] != WakuST::Absolute {
        newh[rx][ry] = [Waku::N, Waku::K, Waku::I, Waku::A, Waku::O, Waku::H][rng.gen_range(0..6)];
        newh
    } else {
        update_randomly1(&hp, &hst, &h)
    }
}

/// ランダムな1つの枠をランダムな枠に変える
fn update_randomly2(hp: &HyouProp, hst: &HyouST, h: &Hyou) -> Hyou {
    let mut newh = h.clone();
    let mut rng = rand::thread_rng();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(0..hp.day_count);
    if hst[rx][ry] != WakuST::Absolute {
        newh[rx][ry] = [Waku::N, Waku::K, Waku::I, Waku::A, Waku::O, Waku::H][rng.gen_range(0..6)];
    }
    newh
}

/// ランダムな1つの枠をN,O,Hのうちランダムな枠に変える Absoluteなら繰り返す
fn update_randomly4(hp: &HyouProp, hst: &HyouST, h: &Hyou) -> Hyou {
    let mut newh = h.clone();
    let mut rng = rand::thread_rng();
    let rx: usize = rng.gen_range(0..hp.worker_count);
    let ry: usize = rng.gen_range(0..hp.day_count);
    let b1 = hst[rx][ry] != WakuST::Absolute;
    // let b2 = h[rx][ry] == Waku::N || h[rx][ry] == Waku::O || h[rx][ry] == Waku::H;
    let b2 = h[rx][ry] == Waku::N || h[rx][ry] == Waku::O || h[rx][ry] == Waku::H || h[rx][ry] == Waku::U;
    if b1 && b2 {
        newh[rx][ry] = [Waku::N, Waku::O, Waku::H][rng.gen_range(0..3)];
        newh
    } else {
        update_randomly4(&hp, &hst, &h)
        //合わない場合表を何個も生成することになるが、このオーバーヘッドはいかほどか
    }
}

/*
各行について
1.  Iが入っていることを確認
2.  ランダムなIを取り除き、Nを代わりに置く
3.  孤立したAを取り除き、Nを代わりに置く
4.  ランダムなKを取り除き、Aを代わりに置く
5.  Iをランダムな位置に追加する
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

fn remove_random(w: Waku, r: usize, hp: &HyouProp, newh: &mut Hyou, rng: &mut ThreadRng) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == w {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = Waku::N;
}

fn add_random(w: Waku, r:usize, hp: &HyouProp, newh: &mut Hyou, rng: &mut ThreadRng) {
    let mut is: Vec<usize> = Vec::new();
    for c in hp.buffer..hp.day_count {
        if newh[r][c] == Waku::N {
            is.push(c);
        }
    }
    let rnd = rng.gen_range(0..is.len());
    newh[r][is[rnd]] = w;
}

/// IAKを破壊せずに入れ替える
fn update_randomly5(hp: &HyouProp, _hst: &HyouST, h: &Hyou) -> Hyou {
    let mut newh = h.clone();
    let mut rng = rand::thread_rng();
    for r in 0..hp.worker_count {
        let i_cnt = count_waku_row!(Waku::I, hp, h, r);
        if i_cnt == 0 {
            remove_random(Waku::K, r, &hp, &mut newh, &mut rng);
            add_random(Waku::K, r, &hp, &mut newh, &mut rng);
        } else {
            remove_random(Waku::I, r, &hp, &mut newh, &mut rng);
            // remove_proper_a()
            remove_random(Waku::K, r, &hp, &mut newh, &mut rng);
            add_random(Waku::I, r, &hp, &mut newh, &mut rng);
            // add_proper_a()
        }

        // if !check() {
            // 戻す
            for c in 0..hp.day_count {
                newh[r][c] = h[r][c];
            }
        // }
    }
    newh
}