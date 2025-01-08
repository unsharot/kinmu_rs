//! 生成した勤務表を標準出力するモジュール

use crate::kinmu_lib::types::{DayState, Schedule, ScheduleProp, ScoreProp, Shift};

use std::fmt;

const ROW_STATS_DIGIT: usize = 2;

/// 表を出力
pub fn print_schedule(schedule_prop: &ScheduleProp, schedule: &Schedule) {
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

    // スコア表示
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
    let f = format!(" {:>stats$}", sum, stats = ROW_STATS_DIGIT);
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
                print!("{}", &num[l..l+1]);
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

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Shift::N => "N",
            Shift::K => "K",
            Shift::I => "I",
            Shift::A => "A",
            Shift::O => "O",
            Shift::H => "H",
            Shift::Y => "Y",
            Shift::D => "D",
            Shift::U => "U",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for DayState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            DayState::Weekday => "W",
            DayState::Holiday => "H",
            DayState::Bath => "B",
            DayState::Bath2 => "2",
            DayState::Measure => "M",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for ScoreProp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ScoreProp::PatternGeneral(p) => format!("PatternGeneral {:?}", p),
            ScoreProp::PatternFixed(p) => format!("PatternFixed {:?}", p),
            ScoreProp::PatternGeneralAny(p) => format!("PatternGeneralAny {:?}", p),
            ScoreProp::PatternFixedAny(p) => format!("PatternFixedAny {:?}", p),
            ScoreProp::Streak(p) => format!("Streak {:?}", p),
            ScoreProp::ShiftsBalance(p) => format!("ShiftsBalance {:?}", p),
            ScoreProp::ShiftHalfBalance(p) => format!("ShiftHalfBalance {:?}", p),
            ScoreProp::ShiftDirPriority(p) => format!("ShiftDirPriority {:?}", p),
            ScoreProp::DayCountRegardStaffAttribute(p) => {
                format!("DayCountRegardStaffAttribute {:?}", p)
            }
            ScoreProp::StaffCountRegardDayAttribute(p) => {
                format!("StaffCountRegardDayAttribute {:?}", p)
            }
            ScoreProp::StaffCount(p) => format!("StaffCount {:?}", p),
            ScoreProp::StaffCountWithPremise(p) => format!("StaffCountWithPremise {:?}", p),
            ScoreProp::NGPair(p) => format!("NGPair {:?}", p),
            ScoreProp::NoSamePair(p) => format!("NoSamePair {:?}", p),
        };
        write!(f, "{}", s)
    }
}
