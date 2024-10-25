use super::types::{
    Waku::*,
    Hyou,
    Score,
    DayST,
    ScoreProp,
    ScoreProp::*,
    HyouProp,
};



use std::collections::HashMap;


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
    let ss: Vec<String> = sps.iter().map(|x| x.to_string()).collect();
    let zipped: Vec<String> = ss.iter().zip(sl.iter()).map(|(x,y)| x.to_string() + ": " + &y.to_string()).collect();
    zipped.join("\n")
}

fn get_score_list(sps: &Vec<ScoreProp>, hp: &HyouProp, h: &Hyou) -> Vec<Score> {
    sps.iter().map(|sp| get_score(hp, h, sp)).collect()
}


fn get_score(hp: &HyouProp, h: &Hyou, sp: &ScoreProp) -> Score {

    match sp {
        IAKrenzoku(p) => check_rows!(iak_renzoku, hp, h, p),
        KIArenzoku(p) => check_rows!(kia_renzoku, hp, h, p),
        KNIArenzoku(p) => check_rows!(knia_renzoku, hp, h, p),
        NNIArenzoku(p) => check_rows!(nnia_renzoku, hp, h, p),
        ONrenzoku(p) => check_rows!(on_renzoku, hp, h, p),
        NHrenzoku(p) => check_rows!(nh_renzoku, hp, h, p),
        OHrenzoku(p) => check_rows!(oh_renzoku, hp, h, p),
        Renkin4(p) => check_rows!(renkin4, hp, h, p),
        Renkin5(p) => check_rows!(renkin5, hp, h, p),
        Renkin6(p) => check_rows!(renkin6, hp, h, p),
        Renkyuu(p) => check_rows!(k_renzoku, hp, h, p),
        Renkyuu2(p) => check_rows!(k_renzoku2, hp, h, p),
        Renkyuu2NoBf(p) => check_rows!(k_renzoku2_no_buffer, hp, h, p),
        OsoHayaBaransu(p) => check_rows!(osohaya_baransu, hp, h, p),
        YakinBaransu(p) => check_rows!(yakin_baransu, hp, h, p),
        OsoBaransu(p) => check_rows!(oso_baransu, hp, h, p),
        HayaBaransu(p) => check_rows!(haya_baransu, hp, h, p),
        KokyuCount(p) => check_rows!(kokyu_count, hp, h, p),
        YakinCount(p) => check_rows!(yakin_count, hp, h, p),
        OsoCount(p) => check_rows!(oso_count, hp, h, p),
        HayaCount(p) => check_rows!(haya_count, hp, h, p),
        // Fukouhei(p) => check_rows!(fukouhei, hp, h, p),
        YakinNinzuu(p) => check_columns!(yakin_ninzuu, hp, h, p),
        NikkinNinzuu(p) => check_columns!(nikkin_ninzuu, hp, h, p),
        OsoNinzuu(p) => check_columns!(oso_ninzuu, hp, h, p),
        HayaNinzuu(p) => check_columns!(haya_ninzuu, hp, h, p),
        NGPair(p) => check_columns!(ng_pair, hp, h, p),
        Leader(p) => check_columns!(leader_ability, hp, h, p),
        YakinAloneWorker(p) => check_columns!(yakin_alone_worker, hp, h, p),
        YakinAloneBeforeFuro(p) => check_columns!(yakin_alone_before_furo, hp, h, p),
        HeyaMoti(p) => check_columns!(heyamoti_ability, hp, h, p),
        NoSamePair3(p) => no_same_pair3(hp, h, p),
        NoSamePair2(p) => no_same_pair2(hp, h, p),
        NoUndef(p) => check_columns!(no_undef, hp, h, p),
        
        _ => 0.0,
    }
}













// trie木を使って連続パターンを検出したい
// まとめて実行できたら早いかも
// 木は初回実行時に構築して保持する

fn iak_renzoku(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut ans = 0.0;
    for i in 0..(hp.day_count - 1) {
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

fn renkin4(hp: &HyouProp, h: &Hyou, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let ws = [N, O, H, I];
    for i in 0..(hp.day_count - 3) {
        if ws.contains(&h[r][i]) && ws.contains(&h[r][i+1]) && ws.contains(&h[r][i+2]) && ws.contains(&h[r][i+3]) {
            if h[r][i+3] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

fn renkin5(hp: &HyouProp, h: &Hyou, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let ws = [N, O, H, I];
    for i in 0..(hp.day_count - 4) {
        if ws.contains(&h[r][i]) && ws.contains(&h[r][i+1]) && ws.contains(&h[r][i+2]) && ws.contains(&h[r][i+3]) && ws.contains(&h[r][i+4]) {
            if h[r][i+4] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

fn renkin6(hp: &HyouProp, h: &Hyou, r: usize, (s1, s2): &(Score, Score)) -> Score {
    let mut ans = 0.0;
    let ws = [N, O, H, I];
    for i in 0..(hp.day_count - 5) {
        if ws.contains(&h[r][i]) && ws.contains(&h[r][i+1]) && ws.contains(&h[r][i+2]) && ws.contains(&h[r][i+3]) && ws.contains(&h[r][i+4]) && ws.contains(&h[r][i+5]) {
            if h[r][i+5] == I {
                ans += s1;
            } else {
                ans += s2;
            }
        }
    }
    ans
}

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


// カウントするタイプのスコアもまとめて実行してから計算したい
// HashMapをつかえそう
// やっても早くなるかはわからない
// HashMapの構築に時間とメモリかかるかも
fn osohaya_baransu(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
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
    let a = d * s;
    a * a
}

fn yakin_baransu(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut cf: isize = 0;
    let mut cl: isize = 0;
    for i in hp.buffer..((hp.day_count - hp.buffer) / 2) {
        if h[r][i] == I {
            cf += 1;
        }
    }
    for i in ((hp.day_count - hp.buffer) / 2)..hp.day_count {
        if h[r][i] == I {
            cl += 1;
        }
    }
    let d = (cf - cl).abs() as Score;
    let a = d * s;
    a * a
}

fn oso_baransu(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut cf: isize = 0;
    let mut cl: isize = 0;
    for i in hp.buffer..((hp.day_count - hp.buffer) / 2) {
        if h[r][i] == O {
            cf += 1;
        }
    }
    for i in ((hp.day_count - hp.buffer) / 2)..hp.day_count {
        if h[r][i] == O {
            cl += 1;
        }
    }
    let d = (cf - cl).abs() as Score;
    let a = d * s;
    a * a
}

fn haya_baransu(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let mut cf: isize = 0;
    let mut cl: isize = 0;
    for i in hp.buffer..((hp.day_count - hp.buffer) / 2) {
        if h[r][i] == H {
            cf += 1;
        }
    }
    for i in ((hp.day_count - hp.buffer) / 2)..hp.day_count {
        if h[r][i] == H {
            cl += 1;
        }
    }
    let d = (cf - cl).abs() as Score;
    let a = d * s;
    a * a
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

fn kokyu_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.workers[r].k_count;
    let cnt = count_waku_row!(K, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn yakin_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.workers[r].i_count;
    let cnt = count_waku_row!(I, hp, h, r);
    let d = (cnt - cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn oso_count(hp: &HyouProp, h: &Hyou, r: usize, s: &Score) -> Score {
    let cnt_needed = hp.workers[r].o_count;
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
    let cnt_needed = hp.workers[r].h_count;
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

fn nikkin_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, (d,cnt_needed,s): &(DayST,isize,Score)) -> Score {
    if hp.days[c] == *d {
        let cnt = count_waku_column!(N, hp, h, c);
        let d = (cnt - cnt_needed).abs() as Score;
        let a = d * s;
        a * a
    } else {
        0.0
    }
}

fn oso_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, (cnt_needed, s): &(isize,Score)) -> Score {
    let cnt = count_waku_column!(O, hp, h, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn haya_ninzuu(hp: &HyouProp, h: &Hyou, c: usize, (cnt_needed, s): &(isize,Score)) -> Score {
    let cnt = count_waku_column!(H, hp, h, c);
    let d = (cnt - *cnt_needed).abs() as Score;
    let a = d * s;
    a * a
}

fn ng_pair(hp: &HyouProp, h: &Hyou, c: usize, s: &Score) -> Score {
    // NGリストにあるペアがIかどうか確認
    let mut ans = 0.0;
    for i in 0..hp.ng_list.len() {
        let (a, b) = hp.ng_list[i];
        if (h[a-1][c] == I && h[b-1][c] == I) || (h[a-1][c] == A && h[b-1][c] == A) {
            ans += s;
        }
    }
    ans
}

fn leader_ability(hp: &HyouProp, h: &Hyou, c: usize, (ab, s): &(isize,Score)) -> Score {
    if matches!(hp.days[c], DayST::Holiday) {
        let mut a_cnt = 0;
        for r in 0..hp.worker_count {
            if (h[r][c] == N) && ((hp.workers[r].ability % ab) != 0) {
                    a_cnt += 1;
            }
        }
        if a_cnt == 0 {
            *s
        } else {
            0.0
        }
    } else {
        0.0
    }
}

///一人で夜勤できるワーカー
fn yakin_alone_worker(hp: &HyouProp, h: &Hyou, c: usize, (ab, s): &(isize,Score)) -> Score {
    let mut i_cnt = 0;
    let mut a_cnt = 0;
    for r in 0..hp.worker_count {
        if h[r][c] == I {
            i_cnt += 1;
            if (hp.workers[r].ability % ab) != 0 {
                a_cnt += 1;
            }
        }
    }
    if (i_cnt == 1) && (a_cnt == 0) {
        *s
    } else {
        0.0
    }
}

fn yakin_alone_before_furo(hp: &HyouProp, h: &Hyou, c: usize, s: &Score) -> Score {
    if hp.days[c - 1] == DayST::Furo {
        let mut i_cnt = 0;
        for r in 0..hp.worker_count {
            if h[r][c] == I {
                    i_cnt += 1;
            }
        }
        if i_cnt <= 1 {
            *s
        } else {
            0.0
        }
    } else {
        0.0
    }
}

fn heyamoti_ability(hp: &HyouProp, h: &Hyou, c: usize, (cnt_needed, ab, s): &(isize,isize,Score)) -> Score {
    let mut a_cnt = 0;
    for r in 0..hp.worker_count {
        if (h[r][c] == N) && ((hp.workers[r].ability % ab) != 0) {
                a_cnt += 1;
        }
    }
    let d = if *cnt_needed > a_cnt {
        (*cnt_needed - a_cnt) as Score
    } else {
        0.0
    };
    s * d * d
}

/// 3回以上同じペアなら発火するスコア
fn no_same_pair3(hp: &HyouProp, h: &Hyou, s: &Score) -> Score {
    let mut map: HashMap<Vec<usize>, isize> = HashMap::new();
    for c in hp.buffer..hp.day_count {
        let mut i_list: Vec<usize> = Vec::new();
        for r in 0..hp.worker_count {
            if matches!(h[r][c], I) {
                i_list.push(r)
            }
        }
        if i_list.len() > 1 {
            *map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for (_, cnt) in &map {
        let a = *cnt - 2;
        if a > 0 {
            ans += (a as Score) * s
        }
    }
    ans
}

/// 2回以上同じペアなら発火するスコア
fn no_same_pair2(hp: &HyouProp, h: &Hyou, s: &Score) -> Score {
    let mut map: HashMap<Vec<usize>, isize> = HashMap::new();
    for c in hp.buffer..hp.day_count {
        let mut i_list: Vec<usize> = Vec::new();
        for r in 0..hp.worker_count {
            if matches!(h[r][c], I) {
                i_list.push(r)
            }
        }
        if i_list.len() > 1 {
            *map.entry(i_list).or_insert(0) += 1;
        }
    }
    let mut ans = 0.0;
    for (_, cnt) in &map {
        let a = *cnt - 1;
        if a > 0 {
            ans += (a as Score) * s
        }
    }
    ans
}

fn no_undef(hp: &HyouProp, h: &Hyou, c: usize, s: &Score) -> Score {
    let cnt = count_waku_column!(U, hp, h, c);
    let d = cnt as Score;
    let a = d * s;
    a * a
}