//! 結果をユーザーに出力するモジュール

mod text;

use crate::kinmu_lib::types::Answer;

use std::io;
// use std::fs::OpenOptions;

pub fn run(ans: Answer) -> Result<(), String> {
    let mut out = io::stdout().lock();
    text::write_answer(&mut out, ans, true).map_err(|e| e.to_string())?;

    // let mut out = OpenOptions::new().create(true).append(true).open("output.txt").map_err(|e| e.to_string())?;
    // text::write_answer(&mut out, ans, false).map_err(|e| e.to_string())?;

    Ok(())
}
