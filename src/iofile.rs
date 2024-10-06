use std::fs::{read_to_string};
use std::io::{Result};

use crate::kata::{HyouProp};


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

// pub fn load_data(_path: &FilePath) -> Result<HyouProp> {
//     let hp = HyouProp {
//         workers: Vec::new(),
//         ng_list: Vec::new(),
//         bounds: (0, 0),
//         days: Vec::new(),
//         buffer: 0,
//         kibou: Vec::new(),
//         k_counts: Vec::new(),
//         i_counts: Vec::new(),
//         o_counts: Vec::new(),
//         h_counts: Vec::new(),
//         i_ninzuu: Vec::new(),
//     };
//     Ok(hp)
// }

///勤務表で使う値を読み込む
pub fn load_hyou_prop(path: &FilePath) -> Result<HyouProp> {
    let contents = read_contents(path)?;

    for line in contents {
        
    }
}

///焼きなましの段階ごとの設定を読み込む
pub fn load_annealing_config(path: &FilePath) -> Result<AnnealingConfig> {
    let contents = read_contents(path)?;
}

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