use std::fs::{read_to_string};
use std::io;

use crate::kata::{
    HyouProp,
    AnnealingConfig,
    Int,
    Waku,
    Worker,
    NGList,
    Days,
    DayST,
    Hyou,
    ScoreProp,
    HyouST,
    WakuST,
};



type FilePath = String;

pub fn load_main_config(path: &FilePath) -> io::Result<Vec<FilePath>> {

    let contents = read_contents(path)?;

    let mut ans: Vec<String> = Vec::new();

    for line in contents {
        if !line.starts_with("--") {
            ans.push(line.to_string());
        }
    }

    Ok(ans)
}

///フィールドごとに区切る
fn sep_by_fields(contents: &Vec<String>) -> Vec<String> {
    let mut temp: Vec<String> = Vec::new();
    let mut ss: Vec<String> = Vec::new();
    for line in contents {
        if line.ends_with(": ") || line.ends_with(":") || line.starts_with("--") {
            ss.push(temp.join("\n"));
            temp = Vec::new();
        } else {
            temp.push(line.to_string());
        }
    }
    ss.push(temp.join("\n"));
    ss
}

///勤務表で使う値を読み込む
pub fn load_config(path: &str) -> io::Result<(HyouProp, Vec<FilePath>, String)> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    let hyou = read_hyou(&ss[7])?;

    let hp = HyouProp {
        workers: read_workers(&ss[1])?,
        ng_list: read_ng_list(&ss[2])?,
        bounds: (read_int(&ss[3])?, read_int(&ss[4])?),
        days: read_days(&ss[5])?,
        buffer: read_int(&ss[6])?,
        kibou: hyou.clone(),
        hyou_st: make_hyou_st(&hyou),
        k_counts: read_ints(&ss[8])?,
        i_counts: read_ints(&ss[9])?,
        o_counts: read_ints(&ss[10])?,
        h_counts: read_ints(&ss[11])?,
        i_ninzuu: read_ints(&ss[12])?,
        seed: read_int(&ss[14])?,
        score_prop: read_score_props(&ss[16])?,
    };
    let fs = ss[15].lines().map(|s| s.to_string()).collect();
    let ff = ss[13].clone(); //fillの関数

    Ok((hp, fs, ff))
    
}

///焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &str) -> io::Result<AnnealingConfig> {
    let contents = read_contents(path)?;

    let ss = sep_by_fields(&contents);

    let ac = AnnealingConfig {
        step: read_int(&ss[1])?, //ここのindexてきとう
        seed: read_int(&ss[2])?,
        score_prop: read_score_props(&ss[3])?,
        update_func: ss[4].clone(),
        max_temp: read_float(&ss[5])?,
        min_temp: read_float(&ss[6])?,
    };

    Ok(ac)
}

/*
HyouPropのなかでもstep,seed,score_propは
アニーリングごとに変わるので別問題
分ける必要があるかも
*/

///ファイルを読み込んで文字列の行ごとの配列を返す関数
fn read_contents(path: &str) -> io::Result<Vec<String>> {

    //ファイルの全文をStringとして読み込む
    let contents = read_to_string(path)?;

    //成形して行ごとのVec<String>にする
    let mut ans: Vec<String> = Vec::new();
    for line in contents.lines() {
        //コメントを除去
        let cleaned_line = match line.find('#') {
            Some(index) => &line[..index],
            None => &line,
        };
        //空白の行を除去
        if cleaned_line != "" {
            ans.push(cleaned_line.to_string());
        }
    }

    Ok(ans)
}



fn read_int(text: &str) -> io::Result<Int> {
    let ans: Int = text.parse::<Int>().unwrap();
    Ok(ans)
}

fn read_ints(text: &str) -> io::Result<Vec<Int>> {
    Ok(text.split_whitespace().map(|x| x.parse::<Int>().unwrap()).collect())
}

fn read_float(text: &str) -> io::Result<f32> {
    let ans: f32 = text.parse::<f32>().unwrap();
    Ok(ans)
}

fn read_workers(text: &str) -> io::Result<Vec<Worker>> {
    let mut ans: Vec<Worker> = Vec::new();
    for line in text.lines() {
        ans.push(read_worker(&line)?);
    }
    Ok(ans)
}

fn read_worker(text: &str) -> io::Result<Worker> {
    // TODO: もうちょっと安全にアクセスしたい
    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    let worker: Worker = Worker {name: words[0].clone(), ability: read_int(&words[1])?};
    Ok(worker)
}

fn read_ng_list(text: &str) -> io::Result<NGList> {
    let mut ans: NGList = Vec::new();
    for line in text.lines() {
        let a: Vec<Int> = line.split_whitespace().map(|x| read_int(x).unwrap()).collect();
        ans.push((a[0], a[1]));
    }
    Ok(ans)
}

fn read_days(text: &str) -> io::Result<Days> {
    Ok(text.chars().map(|c| match c {
        'W' => Ok(DayST::Weekday),
        'H' => Ok(DayST::Holiday),
        'F' => Ok(DayST::Furo),
        '2' => Ok(DayST::Furo2),
        'G' => Ok(DayST::Weight),
        _ => Err("MATCH sinai DAYST desu!!!"),
    }.unwrap()).collect())
}

fn read_hyou(text: &str) -> io::Result<Hyou> {
    let mut ans: Hyou = Vec::new();
    for line in text.lines() {
        println!("{}",line);
        let a: Vec<Waku> = line.chars().map(|c| match c {
            'N' => Ok(Waku::N),
            'K' => Ok(Waku::K),
            'I' => Ok(Waku::I),
            'A' => Ok(Waku::A),
            'O' => Ok(Waku::O),
            'H' => Ok(Waku::H),
            'Y' => Ok(Waku::Y),
            'D' => Ok(Waku::D),
            'U' => Ok(Waku::U),
            ' ' => Ok(Waku::U),
            _ => Err("MATCH sinai WAKU desu!!!")
        }.unwrap()).collect();
        ans.push(a);
    }
    Ok(ans)
}

fn read_score_props(text: &str) -> io::Result<Vec<ScoreProp>> {
    let mut ans: Vec<ScoreProp> = Vec::new();
    for line in text.lines() {
        ans.push(read_score_prop(&line)?);
    }
    Ok(ans)
}

fn read_score_prop(text: &str) -> io::Result<ScoreProp> {
    let words: Vec<&str> = text.split_whitespace().collect();
    // println!("0: {}",words[0]);
    // println!("1: {}",words[1]);
    let prop: ScoreProp = match (words[0], words[1]) {
        ("IAKrenzoku", s) => ScoreProp::IAKrenzoku(read_float(s)?),
        // ("KIAre")
        _ => ScoreProp::NoUndef(0),
    };
    Ok(prop)
}




fn make_hyou_st(h: &Hyou) -> HyouST {
    let mut ans: HyouST = Vec::new();
    for line in h {
        ans.push(line.iter().map(|w| match w {
            Waku::U => WakuST::Random,
            _ => WakuST::Absolute, //Kibouってつかってないんだっけ？
        }).collect());
    }
    ans
}