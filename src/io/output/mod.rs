//! 結果をユーザーに出力するモジュール

mod text;

use crate::kinmu_lib::types::Answer;

use std::io;

pub fn run<W: io::Write>(mut out: &mut W, ans: Answer, use_color: bool) -> Result<(), String> {
    text::write_answer(&mut out, ans, use_color).map_err(|e| e.to_string())
}
