use crate::kata::{
    // Waku,
    Waku::*,
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

macro_rules! check_columns {
    ($check:expr, $hp: expr, $h:expr, $p:expr) => {{
        let mut sum = 0.0;
        for c in $hp.buffer..$hp.day_count {
            sum += $check($hp, $h, c, $p);
        }
        sum
    }};
}

pub fn assess_score(sps: &Vec<ScoreProp>, hp: &HyouProp, h: &Hyou) -> Score {
    get_score_list(sps, hp, h).iter().sum()
}

pub fn show_score(sps: &Vec<ScoreProp>, hp: &HyouProp, h: &Hyou) -> String {
    let sl = get_score_list(sps, hp, h);
    let ss: Vec<String> = sps.iter().map(|x| x.show()).collect();
    let zipped: Vec<String> = ss.iter().zip(sl.iter()).map(|(x,y)| x.to_string() + ": " + &y.to_string()).collect();
    zipped.join("\n")
}

fn get_score_list(sps: &Vec<ScoreProp>, hp: &HyouProp, h: &Hyou) -> Vec<Score> {
    sps.iter().map(|sp| get_score(hp, h, sp)).collect()
}

//Hyou[日][人]とHyou2[人][日]両方保持するのありかも

fn get_score(hp: &HyouProp, h: &Hyou, sp: &ScoreProp) -> Score {

    match sp {
        IAKrenzoku(p) => check_rows!(iak_renzoku, hp, h, p),
        KIArenzoku(p) => check_rows!(kia_renzoku, hp, h, p),
        KNIArenzoku(p) => check_rows!(knia_renzoku, hp, h, p),
        NNIArenzoku(p) => check_rows!(nnia_renzoku, hp, h, p),
        ONrenzoku(p) => check_rows!(on_renzoku, hp, h, p),
        NHrenzoku(p) => check_rows!(nh_renzoku, hp, h, p),
        OHrenzoku(p) => check_rows!(oh_renzoku, hp, h, p),
        
        //略
        
        Renkyuu(p) => check_rows!(k_renzoku, hp, h, p),
        Renkyuu2(p) => check_rows!(k_renzoku2, hp, h, p),
        Renkyuu2NoBf(p) => check_rows!(k_renzoku2_no_buffer, hp, h, p),
        OsoHayaBaransu(p) => check_rows!(osohaya, hp, h, p),

        //略

        KokyuCount(p) => check_rows!(kokyu_count, hp, h, p),
        YakinCount(p) => check_rows!(yakin_count, hp, h, p),
        OsoCount(p) => check_rows!(oso_count, hp, h, p),
        HayaCount(p) => check_rows!(haya_count, hp, h, p),

        //略

        YakinNinzuu(p) => check_columns!(yakin_ninzuu, hp, h, p),
        OsoNinzuu(p) => check_columns!(oso_ninzuu, hp, h, p),
        HayaNinzuu(p) => check_columns!(haya_ninzuu, hp, h, p),

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

fn iak_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        // if (h[r][i] == I) && (h[r][i+1] != A) {
        //     ans += *s;
        // } else if (h[r][i] == A) && !((h[r][i+1] == K) || (h[r][i+1] == Y)) {
        //     ans += *s;
        // } else if (h[r][i] != I) && (h[r][i+1] == A) {
        //     ans += *s;
        // }
        ans += match (h[r][i], h[r][i+1]) {
            (A, K) => 0.0,
            (A, Y) => 0.0,
            (A, _) => *s,
            (I, A) => 0.0,
            (I, _) => *s,
            (_, A) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn kia_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        // if (h[r][i] == K) && (h[r][i+1] == I) {
        //     ans += *s;
        // } else if (h[r][i] == Y) && (h[r][i+1] == I) {
        //     ans += *s;
        // }
        ans += match (h[r][i], h[r][i+1]) {
            (K, I) => *s,
            (Y, I) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn knia_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 2) {
        ans += match (h[r][i], h[r][i+1], h[r][i+2]) {
            (K, N, I) => *s,
            (K, O, I) => *s,
            (K, H, I) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn nnia_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 2) {
        ans += match (h[r][i], h[r][i+1], h[r][i+2]) {
            (N, N, I) => *s,
            (N, O, I) => *s,
            (O, O, I) => *s,
            (H, H, I) => *s,
            (H, N, I) => *s,
            _ => 0.0,
        }
    }
    -ans
}

fn on_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        ans += match (h[r][i], h[r][i+1]) {
            (O, N) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn nh_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        ans += match (h[r][i], h[r][i+1]) {
            (N, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

fn oh_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        ans += match (h[r][i], h[r][i+1]) {
            (O, H) => *s,
            _ => 0.0,
        }
    }
    ans
}

//略

fn k_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
        ans += match (h[r][i], h[r][i+1]) {
            (K, K) => *s,
            (K, Y) => *s,
            (Y, K) => *s,
            (Y, Y) => *s,
            _ => 0.0,
        }
    }
    -ans
}

fn k_renzoku2(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut check = false;
    for i in 0..(hp.day_count - 1) {
        check = check || match (h[r][i], h[r][i+1]) {
            (K, K) => true,
            (K, Y) => true,
            (Y, K) => true,
            (Y, Y) => true,
            _ => false,
        }
    }
    if check {
        0.0
    } else {
        *s
    }
}

fn k_renzoku2_no_buffer(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut check = false;
    for i in hp.buffer..(hp.day_count - 1) {
        check = check || match (h[r][i], h[r][i+1]) {
            (K, K) => true,
            (K, Y) => true,
            (Y, K) => true,
            (Y, Y) => true,
            _ => false,
        }
    }
    if check {
        0.0
    } else {
        *s
    }
}


//カウントするタイプのスコアもまとめて実行してから計算したい
//HashMapをつかえそう
//やっても早くなるかはわからない
//HashMapの構築に時間とメモリかかるかも
fn osohaya(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut oso: isize = 0;
    let mut haya: isize = 0;
    for i in hp.buffer..hp.day_count {
        if h[r][i] == O {
            oso += 1;
        } else if h[r][i] == H {
            haya += 1;
        }
    }
    let d = (haya - oso).abs() as Score;
    d * *s
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

fn kokyu_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.k_counts[r];
    let cnt = count_waku_row!(K, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn yakin_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.i_counts[r];
    let cnt = count_waku_row!(I, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn oso_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.o_counts[r];
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(O, hp, h, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    }
}

fn haya_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.h_counts[r];
    if cnt_needed == -1 {
        0.0
    } else {
        let cnt = count_waku_row!(H, hp, h, r);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    }
}

macro_rules! count_waku_column {
    ($w:expr, $hp: expr, $h:expr, $c:expr) => {{
        let mut cnt: isize = 0;
        for i in 0..$hp.worker_count {
            if $h[i][$c] == $w {
                cnt += 1;
            }
        }
        cnt
    }};
}

fn yakin_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, s: &Score) -> Score {
    let cnt_needed = hp.i_ninzuu[c - hp.buffer];
    let cnt = count_waku_column!(I, hp, h, c);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

//これはdayp(Waku,usize,usize)にしたい
//NikkinNinzuuも(Waku,usize,usize)に
// fn dayP() {}
// fn nikkin_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, s: &((usize,usize),(usize,usize),(usize,usize),(usize,usize),(usize,usize))) -> Score {

// }

fn oso_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, p: &(isize,Score)) -> Score {
    let (cnt_needed, s) = p;
    let cnt = count_waku_column!(O, hp, h, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * *s;
    a * a
}

fn haya_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, p: &(isize,Score)) -> Score {
    let (cnt_needed, s) = p;
    let cnt = count_waku_column!(H, hp, h, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * *s;
    a * a
}

//これもHashMapつかう？
//NGリストをHashMapとして保持して、タプルで検索
// fn ng() {}

//特殊かも
// fn yakinAloneFuro() {}

//これはWorkerとHyouColumnの連携が必須
//結局合計をここで計算する必要あり
// fn heyaMoti(s: &Score, i: &usize, m: &usize, ws: &Vec<Worker>, xs: &HyouColumn) -> Score {
//     let mut c = 0;
//     for _ in 0..xs.len() {
//         if (ws[c].ability % m != 0) && (xs[c] == N) {
//             c += 1;
//         }
//     }
//     let d: f32 = cmp::max(i - c as usize, 0) as f32;
//     return s * d * d;
// }

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