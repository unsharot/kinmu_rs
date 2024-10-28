//! 生成した勤務表を標準出力するモジュール

use crate::kinmu_lib::types::{
    Schedule,
    Shift,
    Shift::*,
    ScheduleProp,
};

const ROW_STATS_DIGIT: usize = 2;

/// 表を出力
pub fn print_schedule(schedule_prop: &ScheduleProp, schedule: &Schedule) {
    for r in 0..schedule_prop.staff_count {
        // Shiftの行を出力
        print_shift_row(schedule_prop, schedule, r);

        // 統計情報
        print_shift_count_row(H, schedule_prop, schedule, r);
        print_shift_count_row(O, schedule_prop, schedule, r);
        print_shift_count_row(I, schedule_prop, schedule, r);
        print_shift_count_row(N, schedule_prop, schedule, r);
        print_shift_count_row(K, schedule_prop, schedule, r);
        print_shift_count_row(Y, schedule_prop, schedule, r);

        // 名前
        print!(" {}", schedule_prop.staff[r].name);

        println!();
    }

    println!();

    // 曜日を表示
    print_days(schedule_prop);

    // 日ごとの統計を表示
    print_shift_count_columns(N, schedule_prop, schedule);
    print_shift_count_columns(I, schedule_prop, schedule);
    print_shift_count_columns(A, schedule_prop, schedule);
    print_shift_count_columns(K, schedule_prop, schedule);
    print_shift_count_columns(O, schedule_prop, schedule);
    print_shift_count_columns(H, schedule_prop, schedule);

    // スコア表示
}

/// Shiftの行を出力
fn print_shift_row(schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize) {
    for c in 0..schedule_prop.day_count{
        print!("{}", schedule[r][c].to_string());
        if c + 1 == schedule_prop.buffer {
            print!("|");
        }
    }
}

/// 指定したシフトの数を出力
fn print_shift_count_row(target_shift: Shift, schedule_prop: &ScheduleProp, schedule: &Schedule, r: usize) {
    let mut sum = 0;
    for c in schedule_prop.buffer..schedule_prop.day_count {
        if schedule[r][c] == target_shift {
            sum += 1;
        }
    }
    // 桁を指定して出力
    let f = format!(" {:>stats$}", sum, stats = ROW_STATS_DIGIT);
    print!("{}", f);
}


/// 曜日を表示
fn print_days(schedule_prop: &ScheduleProp) {
    for c in 0..schedule_prop.day_count {
        print!("{}", schedule_prop.days[c].to_string());
        if c + 1 == schedule_prop.buffer {
            print!("|");
        }
    }
    println!();
}

/// 指定したシフトの列の和を表示
fn print_shift_count_columns(target_shift: Shift, schedule_prop: &ScheduleProp, schedule: &Schedule) {
    let mut v: Vec<String> = Vec::new();
    let mut max_length = 0;
    for c in 0..schedule_prop.day_count {
        let mut sum = 0;
        for r in 0..schedule_prop.staff_count {
            if schedule[r][c] == target_shift {
                sum += 1;
            }
        }
        let s = sum.to_string();
        v.push(s.clone());
        if max_length < s.len() {
            max_length = s.len();
        }
    }
    
    for l in 0..max_length {
        for c in 0..schedule_prop.day_count {
            if l < v[c].len() {
                print!("{}", &v[c][l..l+1]);
            } else {
                print!(" ");
            }
            if c + 1 == schedule_prop.buffer {
                print!("|");
            }
        }
        if l == 0 {
            print!(" {}", target_shift.to_string());
        }
        println!();
    }
}