//! CLI出力のためのモジュール
//! 結果を受け取り、ユーザーに出力する

mod display;

use crate::kinmu_lib::types::Answer;

pub fn show(ans: Answer) {
    for (t, model) in ans.models.iter().enumerate() {
        println!("thread: {}", t + 1);
        display::print_model(&ans.schedule_prop, &model);
    }
    println!();
    println!();
    println!("total time: {:?}", ans.total_time);
}
