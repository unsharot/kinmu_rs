//! 結果をターミナルに標準出力するモジュール

mod display;

use crate::kinmu_lib::score;
use crate::kinmu_lib::types::{Answer, Schedule, ScheduleConfig, Shift};

pub(super) fn print_answer(ans: Answer) {
    for (t, model) in ans.models.iter().enumerate() {
        println!("thread: {}", t + 1);
        print_model(&ans.schedule_config, model);
    }
    println!("total time: {:?}", ans.total_time);
    println!();
}

fn print_model(schedule_config: &ScheduleConfig, model: &Schedule) {
    let score = score::assess_score(
        &mut schedule_config
            .result
            .score_functions
            .iter()
            .flat_map(|x| x.scores.clone())
            .collect(),
        schedule_config,
        model,
    );

    println!("score: {}", score);
    print_schedule(schedule_config, model);

    println!();

    for sf in &schedule_config.result.score_functions {
        let s = score::assess_score(&mut sf.scores.clone(), schedule_config, model);
        println!("{} : {}", sf.display_name, s)
    }

    println!();
}

/// 表を出力
fn print_schedule(schedule_config: &ScheduleConfig, schedule: &Schedule) {
    for r in 0..schedule_config.staff.count {
        // Shiftの行を出力
        print_shift_row(schedule_config, schedule, r);

        // 統計情報
        print_shift_count_row(Shift::H, schedule_config, schedule, r);
        print_shift_count_row(Shift::O, schedule_config, schedule, r);
        print_shift_count_row(Shift::I, schedule_config, schedule, r);
        print_shift_count_row(Shift::N, schedule_config, schedule, r);
        print_shift_count_row(Shift::K, schedule_config, schedule, r);
        print_shift_count_row(Shift::Y, schedule_config, schedule, r);

        // 名前
        print!(" {}", schedule_config.staff.list[r].name);

        println!();
    }

    println!();

    // 曜日を表示
    print_days(schedule_config);

    // 日ごとの統計を表示
    print_shift_count_columns(Shift::N, schedule_config, schedule);
    print_shift_count_columns(Shift::I, schedule_config, schedule);
    print_shift_count_columns(Shift::A, schedule_config, schedule);
    print_shift_count_columns(Shift::K, schedule_config, schedule);
    print_shift_count_columns(Shift::O, schedule_config, schedule);
    print_shift_count_columns(Shift::H, schedule_config, schedule);
}

/// Shiftの行を出力
fn print_shift_row(schedule_config: &ScheduleConfig, schedule: &Schedule, r: usize) {
    for c in 0..schedule_config.day.count {
        print!("{}", schedule[r][c]);
        if c + 1 == schedule_config.day.buffer_count {
            print!("|");
        }
    }
}

/// 指定したシフトの数を出力
fn print_shift_count_row(
    target_shift: Shift,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    r: usize,
) {
    let mut sum = 0;
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if schedule[r][c] == target_shift {
            sum += 1;
        }
    }
    // 桁を指定して出力
    let digit = schedule_config.day.count.to_string().len();
    let f = format!(" {:>stats$}", sum, stats = digit);
    print!("{}", f);
}

/// 曜日を表示
fn print_days(schedule_config: &ScheduleConfig) {
    for c in 0..schedule_config.day.count {
        print!("{}", schedule_config.day.days[c]);
        if c + 1 == schedule_config.day.buffer_count {
            print!("|");
        }
    }
    println!();
}

/// 指定したシフトの列の和を表示
#[allow(clippy::needless_range_loop)]
fn print_shift_count_columns(
    target_shift: Shift,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) {
    // 数値を文字列として保存するベクトル
    let mut str_nums: Vec<String> = Vec::new();
    let mut max_length = 0;
    for c in 0..schedule_config.day.count {
        let mut sum = 0;
        for r in 0..schedule_config.staff.count {
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
            if c + 1 == schedule_config.day.buffer_count {
                print!("|");
            }
        }
        if l == 0 {
            print!(" {}", target_shift);
        }
        println!();
    }
}
