//! 結果をターミナルに標準出力するモジュール

mod display;

use crate::kinmu_lib::score;
use crate::kinmu_lib::types::{Answer, Schedule, ScheduleConfig, Shift};
use crate::utils::color;

use std::io;

pub(super) fn write_answer<W: io::Write>(
    out: &mut W,
    ans: Answer,
    use_color: bool,
) -> io::Result<()> {
    for (t, model) in ans.models.iter().enumerate() {
        writeln!(out, "thread: {}", t + 1)?;
        write_model(out, &ans.schedule_config, model, use_color)?;
    }
    writeln!(out, "total time: {:?}", ans.total_time)?;
    writeln!(out)?;
    Ok(())
}

fn write_model<W: io::Write>(
    out: &mut W,
    schedule_config: &ScheduleConfig,
    model: &Schedule,
    use_color: bool,
) -> io::Result<()> {
    let score = score::eval_scores(
        &mut schedule_config
            .result
            .score_functions
            .iter()
            .flat_map(|x| x.scores.clone())
            .collect(),
        schedule_config,
        model,
    );

    writeln!(out, "score: {}", score)?;
    write_schedule(out, schedule_config, model)?;

    writeln!(out)?;

    for sf in &schedule_config.result.score_functions {
        let s = score::eval_scores(&mut sf.scores.clone(), schedule_config, model);
        let mut ok = true;
        if let Some(f) = &sf.filter {
            if let Some(h) = f.high_pass {
                if s < h {
                    ok = false;
                }
            }
            if let Some(l) = f.low_pass {
                if l < s {
                    ok = false;
                }
            }
        }
        if ok {
            writeln!(out, "{} : {}", sf.display_name, s)?;
        } else {
            write!(out, "{} : {} ", sf.display_name, s)?;
            color::write(out, "[Warning]", color::Color::Red, use_color)?;
            writeln!(out)?;
        }
    }

    writeln!(out)?;

    Ok(())
}

/// 表を出力
fn write_schedule<W: io::Write>(
    out: &mut W,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) -> io::Result<()> {
    for r in 0..schedule_config.staff.count {
        // Shiftの行を出力
        write_shift_row(out, schedule_config, schedule, r)?;

        // 統計情報
        write_shift_count_row(out, Shift::H, schedule_config, schedule, r)?;
        write_shift_count_row(out, Shift::O, schedule_config, schedule, r)?;
        write_shift_count_row(out, Shift::I, schedule_config, schedule, r)?;
        write_shift_count_row(out, Shift::N, schedule_config, schedule, r)?;
        write_shift_count_row(out, Shift::K, schedule_config, schedule, r)?;
        write_shift_count_row(out, Shift::Y, schedule_config, schedule, r)?;

        // 名前
        write!(out, " {}", schedule_config.staff.list[r].name)?;

        writeln!(out)?;
    }

    writeln!(out)?;

    // 曜日を表示
    write_days(out, schedule_config)?;

    // 日ごとの統計を表示
    write_shift_count_columns(out, Shift::N, schedule_config, schedule)?;
    write_shift_count_columns(out, Shift::I, schedule_config, schedule)?;
    write_shift_count_columns(out, Shift::A, schedule_config, schedule)?;
    write_shift_count_columns(out, Shift::K, schedule_config, schedule)?;
    write_shift_count_columns(out, Shift::O, schedule_config, schedule)?;
    write_shift_count_columns(out, Shift::H, schedule_config, schedule)?;

    Ok(())
}

/// Shiftの行を出力
fn write_shift_row<W: io::Write>(
    out: &mut W,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    r: usize,
) -> io::Result<()> {
    for c in 0..schedule_config.day.count {
        write!(out, "{}", schedule[r][c])?;
        if c + 1 == schedule_config.day.buffer_count {
            write!(out, "|")?;
        }
    }

    Ok(())
}

/// 指定したシフトの数を出力
fn write_shift_count_row<W: io::Write>(
    out: &mut W,
    target_shift: Shift,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
    r: usize,
) -> io::Result<()> {
    let mut sum = 0;
    for c in schedule_config.day.buffer_count..schedule_config.day.count {
        if schedule[r][c] == target_shift {
            sum += 1;
        }
    }
    // 桁を指定して出力
    let digit = schedule_config.day.count.to_string().len();
    let f = format!(" {:>stats$}", sum, stats = digit);
    write!(out, "{}", f)?;

    Ok(())
}

/// 曜日を表示
fn write_days<W: io::Write>(out: &mut W, schedule_config: &ScheduleConfig) -> io::Result<()> {
    for c in 0..schedule_config.day.count {
        write!(out, "{}", schedule_config.day.days[c])?;
        if c + 1 == schedule_config.day.buffer_count {
            write!(out, "|")?;
        }
    }
    writeln!(out)?;

    Ok(())
}

/// 指定したシフトの列の和を表示
#[allow(clippy::needless_range_loop)]
fn write_shift_count_columns<W: io::Write>(
    out: &mut W,
    target_shift: Shift,
    schedule_config: &ScheduleConfig,
    schedule: &Schedule,
) -> io::Result<()> {
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
                write!(out, "{}", &num[l..l + 1])?;
            } else {
                write!(out, " ")?;
            }
            if c + 1 == schedule_config.day.buffer_count {
                write!(out, "|")?;
            }
        }
        if l == 0 {
            write!(out, " {}", target_shift)?;
        }
        writeln!(out)?;
    }

    Ok(())
}
