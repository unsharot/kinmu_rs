use std::fs::{read_to_string};
use std::io::{Result};

use crate::kata::{HyouProp, AnnealingConfig};


/*
FilePath -> Stringの変換にはread_to_stringを使う
*/

type FilePath = String;

pub fn load_main_config(path: &FilePath) -> Result<Vec<FilePath>> {

    let contents = read_contents(path)?;

    let mut ans: Vec<String> = Vec::new();

    for line in contents {
        if !line.starts_with("--") {
            ans.push(line.to_string());
        }
    }

    Ok(ans)
}

///勤務表で使う値を読み込む
pub fn load_config(path: &FilePath) -> Result<(HyouProp, Vec<FilePath>, String)> {
    let contents = read_contents(path)?;

    //フィールドごとに区切る
    let mut temp: Vec<String> = Vec::new();
    let mut ss: Vec<String> = Vec::new();
    for line in contents {
        if line.ends_with(": ") || line.ends_with(":") || line.starts_with("--") {
            ss.push(temp.join("\n"));
            temp = Vec::new();
        } else {
            temp.push(line);
        }
    }

    let hp = HyouProp {
        workers: read_workers(ss[0])?,
        ng_list: read_ng_list(ss[1])?,
        bounds: (read_int(ss[2])?, read_int(ss[3])?),
        days: read_days(ss[4])?,
        buffer: read_int(ss[5])?,
        kibou: read_hyou(ss[6])?,
        k_counts: read_ints(ss[7])?,
        i_counts: read_ints(ss[8])?,
        o_counts: read_ints(ss[9])?,
        h_counts: read_ints(ss[10])?,
        i_ninzuu: read_ints(ss[11])?,
        seed: read_int(ss[13])?,
        score_prop: read_score_prop(ss[15])?,
    };
    let fs = ss[14].lines();
    let ff = ss[12];

    Ok((hp, fs, ff))
    
}

// ///焼きなましの段階ごとの設定を読み込む
// pub fn load_annealing_config(path: &FilePath) -> Result<AnnealingConfig> {
//     let contents = read_contents(path)?;
// }

/*
HyouPropのなかでもstep,seed,score_propは
アニーリングごとに変わるので別問題
分ける必要があるかも
*/

///ファイルを読み込んで文字列の行ごとの配列を返す関数
fn read_contents(path: &FilePath) -> Result<Vec<String>> {

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

