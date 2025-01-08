//! 結果をユーザーに出力するモジュール

mod display;

use crate::kinmu_lib::types::Answer;

pub fn run(ans: Answer) {
    display::print_answer(ans);
}
