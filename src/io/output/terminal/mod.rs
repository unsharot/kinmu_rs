//! 結果をターミナルに標準出力するモジュール

mod display;

use crate::kinmu_lib::score;
use crate::kinmu_lib::types::{Answer, Schedule, ScheduleProp, Shift};

pub(super) fn print_answer(ans: Answer) {
    for (t, model) in ans.models.iter().enumerate() {
        println!("thread: {}", t + 1);
        print_model(&ans.schedule_prop, model);
    }
    println!("total time: {:?}", ans.total_time);
    println!();
}

fn print_model(schedule_prop: &ScheduleProp, model: &Schedule) {
    let score = score::assess_score(&mut schedule_prop.score_props.clone(), schedule_prop, model);

    println!("score: {}", score);
    print_schedule(schedule_prop, model);

    println!();

    println!(
        "{}",
        score::show_score(&mut schedule_prop.score_props.clone(), schedule_prop, model)
    );
}

/// 表を出力
fn print_schedule(schedule_prop: &ScheduleProp, schedule: &Schedule) {
    for r in 0..schedule_prop.staff_count {
        // Shiftの行を出力
        print_shift_row(schedule_prop, schedule, r);

        // 統計情報
        print_shift_count_row(Shift::H, schedule_prop, schedule, r);
        print_shift_count_row(Shift::O, schedule_prop, schedule, r);
        print_shift_count_row(Shift::I, schedule_prop, schedule, r);
        print_shift_count_row(Shift::N, schedule_prop, schedule, r);
        print_shift_count_row(Shift::K, schedule_prop, schedule, r);
        print_shift_count_row(Shift::Y, schedule_prop, schedule, r);

        // 名前
        print!(" {}", schedule_prop.staff_list[r].name);

        println!();
    }

    println!();

    // 曜日を表示
    print_days(schedule_prop);

    // 日ごとの統計を表示
    print_shift_count_columns(Shift::N, schedule_prop, schedule);
    print_shift_count_columns(Shift::I, schedule_prop, schedule);
    print_shift_count_columns(Shift::A, schedule_prop, schedule);
    print_shift_count_columns(Shift::K, schedule_prop, schedule);
    print_shift_count_columns(Shift::O, schedule_prop, schedule);
    print_shift_count_columns(Shift::H, schedule_prop, schedule);
}

/// Shiftの行を出力
fn print_shift_row(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize) {
    for c in 0..schedule_prop.day_count {
        print!("{}", schedule[r][c]);
        if c + 1 == schedule_prop.buffer {
            print!("|");
        }
    }
}

/// 指定したシフトの数を出力
fn print_shift_count_row(
    target_shift: Shift,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
    r: usize,
) {
    let mut sum = 0;
    for c in schedule_prop.buffer..schedule_prop.day_count {
        if schedule[r][c] == target_shift {
            sum += 1;
        }
    }
    // 桁を指定して出力
    let digit = schedule_prop.day_count.to_string().len();
    let f = format!(" {:>stats$}", sum, stats = digit);
    print!("{}", f);
}

/// 曜日を表示
fn print_days(schedule_prop: &ScheduleProp) {
    for c in 0..schedule_prop.day_count {
        print!("{}", schedule_prop.days[c]);
        if c + 1 == schedule_prop.buffer {
            print!("|");
        }
    }
    println!();
}

/// 指定したシフトの列の和を表示
#[allow(clippy::needless_range_loop)]
fn print_shift_count_columns(
    target_shift: Shift,
    schedule_prop: &ScheduleProp,
    schedule: &Schedule,
) {
    // 数値を文字列として保存するベクトル
    let mut str_nums: Vec<String> = Vec::new();
    let mut max_length = 0;
    for c in 0..schedule_prop.day_count {
        let mut sum = 0;
        for r in 0..schedule_prop.staff_count {
            if schedule[r][c] == target_shift {
                sum += 1;
            }
        }
        let s = sum.to_string();
        str_nums.push(s.clone());
        if max_length < s.len() {
            max_length = s.len();
        }
    }

    for l in 0..max_length {
        for (c, num) in str_nums.iter().enumerate() {
            if l < num.len() {
                print!("{}", &num[l..l + 1]);
            } else {
                print!(" ");
            }
            if c + 1 == schedule_prop.buffer {
                print!("|");
            }
        }
        if l == 0 {
            print!(" {}", target_shift);
        }
        println!();
    }
}
