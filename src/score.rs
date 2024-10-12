use crate::kata::{
    Waku,
    Hyou,
    // HyouRow,
    // HyouColumn,
    Score,
    // DayST,
    // NG,
    // NGList,
    ScoreProp,
    ScoreProp::*,
    HyouProp,
    // Worker,
};



// use std::cmp;


macro_rules! check_rows {
    ($check:expr, $hp: expr, $h:expr, $p:expr) => {{
        let mut sum = 0.0;
        for r in 0..$hp.worker_count {
            sum += $check($hp, $h, r, $p);
        }
        sum
    }};
}

pub fn assess_score(hp: &HyouProp, h: &Hyou) -> Score {
    get_score_list(hp, h).iter().sum()
}

pub fn show_score(hp: &HyouProp, h: &Hyou) -> String {
    let sps = &hp.score_prop;
    let sl = get_score_list(hp, h);
    let ss: Vec<String> = sps.iter().map(|x| x.show()).collect();
    let zipped: Vec<String> = ss.iter().zip(sl.iter()).map(|(x,y)| x.to_string() + &y.to_string()).collect();
    zipped.join("\n")
}

impl ScoreProp {
    fn show(&self) -> String {
        match self {
            IAKrenzoku(s) => "IAKrenzoku ".to_owned() + &s.to_string(),
            _ => "NO WAY!!!".to_string(),
        }
    }
}

fn get_score_list(hp: &HyouProp, h: &Hyou) -> Vec<Score> {
    let sps = &hp.score_prop;
    sps.iter().map(|sp| get_score(hp, h, sp)).collect()
}

//Hyou[日][人]とHyou2[人][日]両方保持するのありかも

fn get_score(hp: &HyouProp, h: &Hyou, sp: &ScoreProp) -> Score {

    match sp {
        // IAKrenzoku(p) => check_rows!(renzoku, hp, h, p),
        // KIArenzoku(p) => 0.0, //check_rows!(renzoku, hp, h, p),
        // KNIArenzoku(p) => 0.0, //check_rows!(renzoku, hp, h, p)
        // NNIArenzoku(p) => 0.0, //check_rows!(renzoku, hp, h, p)
        
        //略
        
        OsoHayaBaransu(p) => check_rows!(osohaya, hp, h, p),

        //略

        KokyuCount(p) => check_rows!(kokyu_count, hp, h, p),
        YakinCount(p) => check_rows!(yakin_count, hp, h, p),
        OsoCount(p) => check_rows!(oso_count, hp, h, p),
        HayaCount(p) => check_rows!(haya_count, hp, h, p),

        _ => 0.0,
        // _ => {println!("MATCH SINAI SP DESU!!! (score)"); 0.0},
    }
}
//hpのパラメータはhp._でとる
//遅延評価がデフォじゃないので


//trie木を使って連続パターンを検出したい
//まとめて実行できたら早いかも
//木は初回実行時に構築して保持する
// fn renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
//     // for i in 0..hp.day_count {
//     //     h[r][i]
//     // }
//     0.0
// }

//カウントするタイプのスコアもまとめて実行してから計算したい
//HashMapをつかえそう
//やっても早くなるかはわからない
//HashMapの構築に時間とメモリかかるかも
fn osohaya(hp: &HyouProp, h: &Hyou, r: usize, m: &isize) -> Score {
    let mut oso: isize = 0;
    let mut haya: isize = 0;
    for i in 0..hp.day_count {
        if h[r][i] == Waku::O {
            oso += 1;
        } else if h[r][i] == Waku::H {
            haya += 1;
        }
    }
    let d = (haya - oso).abs();
    (d * *m) as Score
}

// fn yakinBaransu() {}

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

fn kokyu_count(hp: &HyouProp, h: &Hyou, r: usize, m: &Score) -> Score {
    let cnt_needed = hp.k_counts[r] as isize;
    let cnt = count_waku_row!(Waku::K, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * m;
    a * a
}

fn yakin_count(hp: &HyouProp, h: &Hyou, r: usize, m: &Score) -> Score {
    let cnt_needed = hp.i_counts[r] as isize;
    let cnt = count_waku_row!(Waku::I, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * m;
    a * a
}

fn oso_count(hp: &HyouProp, h: &Hyou, r: usize, m: &Score) -> Score {
    let cnt_needed = hp.o_counts[r] as isize;
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(Waku::O, hp, h, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * m;
        a * a
    }
}

fn haya_count(hp: &HyouProp, h: &Hyou, r: usize, m: &Score) -> Score {
    let cnt_needed = hp.h_counts[r] as isize;
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(Waku::H, hp, h, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * m;
        a * a
    }
}

//これはdayp(Waku,usize,usize)にしたい
//NikkinNinzuuも(Waku,usize,usize)に
// fn dayP() {}


//これはWorkerとHyouColumnの連携が必須
//結局合計をここで計算する必要あり
// fn heyaMoti(s: &Score, i: &usize, m: &usize, ws: &Vec<Worker>, xs: &HyouColumn) -> Score {
//     let mut c = 0;
//     for _ in 0..xs.len() {
//         if (ws[c].ability % m != 0) && (xs[c] == Waku::N) {
//             c += 1;
//         }
//     }
//     let d: f32 = cmp::max(i - c as usize, 0) as f32;
//     return s * d * d;
// }

//これもHashMapつかう？
//NGリストをHashMapとして保持して、タプルで検索
// fn ng() {}

//特殊かも
// fn yakinAloneFuro() {}

//日ごとにペアを出して、その重複を調べる
//HashMap使えそう
// fn noSamePair(s: &Score, cs: &Vec<HyouColumn>) -> Score {
    // 0.0
// }


/*
この形式を使うことにする

fn basic_row_score_func(hp: &HyouProp, h: &Hyou, r: usize, p: &T) -> Score {
    for i in 0..hp.day_count {
        h[r][i]
    }
}

fn basic_column_score_func(hp: &HyouProp, h: &Hyou, c: usize, p: &T) -> Score {
    for i in 0..hp.worker_count {
        h[i][c]
    }
}

*/