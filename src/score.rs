mod kata;
use kata::{
    Waku,
    Hyou,
    HyouRow,
    HyouColumn,
    Score,
    DayST,
    NG,
    NGList,
    ScoreProp,
    HyouProp,
    Worker,
};






pub fn assess_score(hp: &HyouProp, h: &Hyou) -> Score {
    get_score_list(hp, h).iter().sum()
}

pub fn show_score(hp: &HyouProp, h: &Hyou) -> String {
    let sps = hp.score_prop;
    let sl = get_score_list(hp, h);
    let ss = sps.iter().map(|x| show(x)).collect();
    let zipped = ss.iter().zip(sl.iter()).collect();
    show(zipped)
}

fn get_score_list(hp: &HyouProp, h: &Hyou) -> Vec<Score> {
    let sps = hp.score_prop;
    let rs = sepToRow(h);
    let cs = sepToColumn(h);
    sps.iter().map(|sp| get_score(hp, h, sp, rs, cs)).collect()
}

//Hyou[日][人]とHyou2[人][日]両方保持するのありかも

fn get_score(hp: &HyouProp, h: &Hyou, sp: &ScoreProp, rs: &Vec<HyouRow>, cs: &Vec<HyouColumn>) -> Score {

    match sp {
        IAKrenzoku(s) => checkRow()
    }
}
//hpのパラメータはhp._でとる
//遅延評価がデフォじゃないので


//trie木を使って連続パターンを検出したい
//まとめて実行できたら早いかも
//木は初回実行時に構築して保持する
fn renzoku(s: &Score, r: &HyouRow) -> Score {

}

//カウントするタイプのスコアもまとめて実行してから計算したい
//HashMapをつかえそう
//やっても早くなるかはわからない
//HashMapの構築に時間とメモリかかるかも
fn osohaya(m: &isize, xs: &HyouRow) -> Score {

}

fn yakinBaransu()

fn kokyuCountP()


//これはdayp(Waku,Int,Int)にしたい
//NikkinNinzuuも(Waku,Int,Int)に
fn dayP()


//これはWorkerとHyouColumnの連携が必須
//結局合計をここで計算する必要あり
fn heyaMoti(s: &Score, i: &Int, m: &Int, ws: &Vec<Worker>, xs: &HyouColumn) -> Score {
    let mut c = 0;
    for i in 0..xs.len() {
        if (ws[c].ability % m != 0) && (xs[c] == Waku::N) {
            c += 1;
        }
    }
    let d = max(i - c, 0);
    return s * d * d;
}

//これもHashMapつかう？
//NGリストをHashMapとして保持して、タプルで検索
fn ng()

//特殊かも
fn yakinAloneFuro()

//日ごとにペアを出して、その重複を調べる
//HashMap使えそう
fn noSamePair(s: &Score, cs: &Vec<HyouColumn>) -> Score {

}