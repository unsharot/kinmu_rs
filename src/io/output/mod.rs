//! 結果をユーザーに出力するモジュール

mod terminal;

use crate::kinmu_lib::types::Answer;

pub fn run(ans: Answer) {
    terminal::print_answer(ans);
}
