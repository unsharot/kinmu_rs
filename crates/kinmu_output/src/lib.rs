//! 結果をユーザーに出力するモジュール

use ::kinmu_color;
use ::kinmu_core::Output;
use ::kinmu_model::{eval_scores_immut, Answer, Schedule, ScheduleConfig, ScorePropTrait};

use std::fmt;
use std::io;

#[derive(Debug)]
pub struct OutputText<'a, W: io::Write, S> {
    out: &'a mut W,
    use_color: bool,
    row_stats_shifts: Vec<S>,
    column_stats_shifts: Vec<S>,
}

impl<'a, W: io::Write, S> OutputText<'a, W, S> {
    pub fn new(
        out: &'a mut W,
        use_color: bool,
        row_stats_shifts: Vec<S>,
        column_stats_shifts: Vec<S>,
    ) -> Self {
        OutputText {
            out,
            use_color,
            row_stats_shifts,
            column_stats_shifts,
        }
    }
}

impl<W, SP, S, SS, DS> Output<Vec<Answer<SP, S, SS, DS>>> for OutputText<'_, W, S>
where
    W: io::Write,
    SP: ScorePropTrait<S, SS, DS> + Clone,
    S: fmt::Display + PartialEq + Clone,
    DS: fmt::Display,
{
    fn run(&mut self, answers: &Vec<Answer<SP, S, SS, DS>>) -> anyhow::Result<()> {
        for ans in answers {
            self.write_answer(ans)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        }
        Ok(())
    }
}

impl<W, S> OutputText<'_, W, S>
where
    W: io::Write,
    S: fmt::Display + PartialEq,
{
    fn write_answer<SP, SS, DS>(&mut self, ans: &Answer<SP, S, SS, DS>) -> io::Result<()>
    where
        S: Clone,
        SP: ScorePropTrait<S, SS, DS> + Clone,
        DS: fmt::Display,
    {
        for (t, model) in ans.models.iter().enumerate() {
            writeln!(self.out, "thread: {}", t + 1)?;
            self.write_model(&ans.schedule_config, model)?;
        }
        writeln!(self.out, "total time: {:?}", ans.total_time)?;
        writeln!(self.out)?;
        Ok(())
    }

    fn write_model<SP, SS, DS>(
        &mut self,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        model: &Schedule<S>,
    ) -> io::Result<()>
    where
        S: Clone,
        SP: ScorePropTrait<S, SS, DS> + Clone,
        DS: fmt::Display,
    {
        let score = eval_scores_immut(
            &schedule_config
                .result
                .score_functions
                .iter()
                .flat_map(|x| x.scores.clone())
                .collect::<Vec<SP>>(),
            schedule_config,
            model,
        );

        writeln!(self.out, "score: {}", score)?;
        self.write_schedule(schedule_config, model)?;

        writeln!(self.out)?;

        for sf in &schedule_config.result.score_functions {
            let s = eval_scores_immut(&sf.scores, schedule_config, model);
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
                writeln!(self.out, "{} : {}", sf.display_name, s)?;
            } else {
                write!(self.out, "{} : {} ", sf.display_name, s)?;
                kinmu_color::write(
                    self.out,
                    "[Warning]",
                    kinmu_color::Color::Red,
                    self.use_color,
                )?;
                writeln!(self.out)?;
            }
        }

        writeln!(self.out)?;

        Ok(())
    }

    /// 表を出力
    fn write_schedule<SP, SS, DS>(
        &mut self,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> io::Result<()>
    where
        S: Clone,
        DS: fmt::Display,
    {
        for r in 0..schedule_config.staff.count {
            // Shiftの行を出力
            self.write_shift_row(schedule_config, schedule, r)?;

            // 統計情報
            for s in 0..self.row_stats_shifts.len() {
                self.write_shift_count_row(s, schedule_config, schedule, r)?;
            }

            // 名前
            write!(self.out, " {}", schedule_config.staff.list[r].name)?;

            writeln!(self.out)?;
        }

        writeln!(self.out)?;

        // 曜日を表示
        self.write_days(schedule_config)?;

        // 日ごとの統計を表示
        for s in 0..self.column_stats_shifts.len() {
            self.write_shift_count_columns(s, schedule_config, schedule)?;
        }

        Ok(())
    }

    /// Shiftの行を出力
    fn write_shift_row<SP, SS, DS>(
        &mut self,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        schedule: &Schedule<S>,
        r: usize,
    ) -> io::Result<()>
    where
        DS: fmt::Display,
    {
        for c in 0..schedule_config.day.count {
            write!(self.out, "{}", schedule[r][c])?;
            if c + 1 == schedule_config.day.buffer_count {
                write!(self.out, "|")?;
            }
        }

        Ok(())
    }

    /// 指定したシフトの数を出力
    fn write_shift_count_row<SP, SS, DS>(
        &mut self,
        index: usize,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        schedule: &Schedule<S>,
        r: usize,
    ) -> io::Result<()>
    where
        DS: fmt::Display,
    {
        let mut sum = 0;
        for c in schedule_config.day.buffer_count..schedule_config.day.count {
            if schedule[r][c] == self.row_stats_shifts[index] {
                sum += 1;
            }
        }
        // 桁を指定して出力
        let digit = schedule_config.day.count.to_string().len();
        let f = format!(" {:>stats$}", sum, stats = digit);
        write!(self.out, "{}", f)?;

        Ok(())
    }

    /// 曜日を表示
    fn write_days<SP, SS, DS>(
        &mut self,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
    ) -> io::Result<()>
    where
        DS: fmt::Display,
    {
        for c in 0..schedule_config.day.count {
            write!(self.out, "{}", schedule_config.day.days[c])?;
            if c + 1 == schedule_config.day.buffer_count {
                write!(self.out, "|")?;
            }
        }
        writeln!(self.out)?;

        Ok(())
    }

    /// 指定したシフトの列の和を表示
    #[allow(clippy::needless_range_loop)]
    fn write_shift_count_columns<SP, SS, DS>(
        &mut self,
        index: usize,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        schedule: &Schedule<S>,
    ) -> io::Result<()>
    where
        DS: fmt::Display,
    {
        // 数値を文字列として保存するベクトル
        let mut str_nums: Vec<String> = Vec::new();
        let mut max_length = 0;
        for c in 0..schedule_config.day.count {
            let mut sum = 0;
            for r in 0..schedule_config.staff.count {
                if schedule[r][c] == self.column_stats_shifts[index] {
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
                    write!(self.out, "{}", &num[l..l + 1])?;
                } else {
                    write!(self.out, " ")?;
                }
                if c + 1 == schedule_config.day.buffer_count {
                    write!(self.out, "|")?;
                }
            }
            if l == 0 {
                write!(self.out, " {}", self.column_stats_shifts[index])?;
            }
            writeln!(self.out)?;
        }

        Ok(())
    }
}
