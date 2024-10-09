use crate::kata::{
    Waku,
    Hyou,
    WakuST,
    HyouST,
    // HyouProp,
};

use rand::Rng;


pub fn gen_update_func<'a>(text: &str, hst: &'a HyouST) -> Box<dyn FnMut(&Hyou) -> Hyou + 'a> {
    match text {
        _ => Box::new(move |h| update_randomly4(hst, h))
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

pub fn update_randomly4(hst: &HyouST, h: &Hyou) -> Hyou {
    let mut newh = h.clone();
    let mut rng = rand::thread_rng();
    let rx: usize = rng.gen_range(0..h.len());
    let ry: usize = rng.gen_range(0..h[0].len());
    let b1 = hst[rx][ry] != WakuST::Absolute;
    let b2 = h[rx][ry] == Waku::N || h[rx][ry] == Waku::O || h[rx][ry] == Waku::H;
    if b1 && b2 {
        newh[rx][ry] = [Waku::N, Waku::O, Waku::H][rng.gen_range(0..3)];
        newh
    } else {
        update_randomly4(&hst, &h)
    }
}

// pub fn update_randomly5() {}