//! html出力機能を提供
//! ファイル出力と標準出力は引数で切り替え

use kinmu_core::Output;
use kinmu_model::{eval_scores_immut, Answer, Schedule, ScheduleConfig, ScoreProp};

use std::fmt;
use std::io;

/// テキスト出力の出力器
/// outは出力先
/// row_stats_shiftsは行の統計を表示するシフト
/// column_stats_shiftsは列の統計を表示するシフト
#[derive(Debug)]
pub struct OutputHTML<'a, W: io::Write, S> {
    out: &'a mut W,
    row_stats_shifts: Vec<S>,
    column_stats_shifts: Vec<S>,
}

impl<'a, W: io::Write, S> OutputHTML<'a, W, S> {
    /// コンストラクタ
    /// outは出力先
    /// row_stats_shiftsは行の統計を表示するシフト
    /// column_stats_shiftsは列の統計を表示するシフト
    pub fn new(out: &'a mut W, row_stats_shifts: Vec<S>, column_stats_shifts: Vec<S>) -> Self {
        OutputHTML {
            out,
            row_stats_shifts,
            column_stats_shifts,
        }
    }
}

/// 出力器の実装
/// 出力のため、一部の型にfmt::Displayを要求
/// また、処理上の都合でCloneも要求
impl<W, SP, S, SS, DS> Output<Vec<Answer<SP, S, SS, DS>>> for OutputHTML<'_, W, S>
where
    W: io::Write,
    SP: ScoreProp<S, SS, DS> + Clone,
    S: fmt::Display + PartialEq + Clone,
    DS: fmt::Display,
{
    fn run(&mut self, answers: &Vec<Answer<SP, S, SS, DS>>) -> anyhow::Result<()> {
        for ans in answers {
            self.write_answer(ans)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        }
        self.write_style()?;
        Ok(())
    }
}

impl<W, S> OutputHTML<'_, W, S>
where
    W: io::Write,
    S: fmt::Display + PartialEq,
{
    /// self.outにAnswerを出力
    /// ただし、戻り値はio::Result<()>であることに注意
    fn write_answer<SP, SS, DS>(&mut self, ans: &Answer<SP, S, SS, DS>) -> io::Result<()>
    where
        S: Clone,
        SP: ScoreProp<S, SS, DS> + Clone,
        DS: fmt::Display,
    {
        for (t, model) in ans.models.iter().enumerate() {
            writeln!(self.out, "<div>thread: {}</div>", t + 1)?;
            self.write_model(&ans.schedule_config, model)?;
            writeln!(self.out, "<br/>")?;
        }
        writeln!(self.out, "<div>total time: {:?}</div>", ans.total_time)?;
        writeln!(self.out)?;
        Ok(())
    }

    /// 1つの表を統計情報やスコア含めて出力
    fn write_model<SP, SS, DS>(
        &mut self,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        model: &Schedule<S>,
    ) -> io::Result<()>
    where
        S: Clone,
        SP: ScoreProp<S, SS, DS> + Clone,
        DS: fmt::Display,
    {
        let score = eval_scores_immut(
            &schedule_config
                .result
                .score_functions
                .iter()
                .flat_map(|x| x.scores.clone())
                .collect::<Vec<SP>>(),
            &schedule_config.staff,
            &schedule_config.day,
            model,
        );

        writeln!(self.out, "<div>score: {}</div>", score)?;
        self.write_schedule(schedule_config, model)?;

        writeln!(self.out)?;

        for sf in &schedule_config.result.score_functions {
            let s = eval_scores_immut(
                &sf.scores,
                &schedule_config.staff,
                &schedule_config.day,
                model,
            );
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
                writeln!(self.out, "<div>{} : {}</div>", sf.display_name, s)?;
            } else {
                write!(
                    self.out,
                    "<div>{} : {} [Warning]</div> ",
                    sf.display_name, s
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
        write!(self.out, "<table>")?;

        write!(self.out, "<thead><tr>")?;
        write!(self.out, "<th scope=\"col\">人/日付</th>")?;

        for c in 0..schedule_config.day.count {
            write!(
                self.out,
                "<th scope=\"col\"><div>{}</div><div>{}</div></th>",
                schedule_config.day.days[c], c
            )?;
        }

        write!(self.out, "<th scope=\"col\" class=\"padding\"></th>")?;

        for s in 0..self.row_stats_shifts.len() {
            write!(
                self.out,
                "<th scope=\"col\">{}</th>",
                self.row_stats_shifts[s]
            )?;
        }

        write!(self.out, "</tr></thead>")?;

        write!(self.out, "<tbody>")?;

        for r in 0..schedule_config.staff.count {
            write!(self.out, "<tr>")?;

            // 名前
            write!(
                self.out,
                "<th scope=\"row\">{}</th>",
                schedule_config.staff.list[r].name
            )?;

            // Shiftの行を出力
            self.write_shift_row(schedule_config, schedule, r)?;

            write!(self.out, "<td class=\"padding\"/>")?;

            // 行の統計情報
            for s in 0..self.row_stats_shifts.len() {
                self.write_shift_count_row(s, schedule_config, schedule, r)?;
            }

            write!(self.out, "</tr>")?;
        }

        write!(self.out, "</tbody>")?;

        write!(self.out, "<thead><tr><td class=\"padding\"></td></tr></thead>")?;

        write!(self.out, "<tfoot>")?;

        // 曜日を表示
        self.write_days(schedule_config)?;

        // 列の統計を表示
        for s in 0..self.column_stats_shifts.len() {
            self.write_shift_count_columns(s, schedule_config, schedule)?;
        }

        write!(self.out, "</tfoot>")?;

        write!(self.out, "</table>")?;
        Ok(())
    }

    /// 表のrで指定した行を出力
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
            write!(self.out, "<td>{}</td>", schedule[r][c])?;
        }

        Ok(())
    }

    /// 行のShiftの統計を出力
    /// row_stats_shiftsのindexで指定したシフトの数をr行目について出力
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

        write!(self.out, "<td>{}</td>", sum)?;

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
        write!(self.out, "<tr>")?;
        write!(self.out, "<th scope=\"col\">シフト/日付</th>")?;

        for c in 0..schedule_config.day.count {
            write!(
                self.out,
                "<th scope=\"col\"><div>{}</div><div>{}</div></th>",
                schedule_config.day.days[c], c
            )?;
        }

        write!(self.out, "</tr>")?;

        Ok(())
    }

    /// column_stats_shiftsのindexで指定した列の和を表示
    /// row_stats_shiftsのindexで指定したシフトの数を各列について出力
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
        write!(self.out, "<tr>")?;

        write!(
            self.out,
            "<th scope=\"row\">{}</th>",
            self.column_stats_shifts[index]
        )?;
        for c in 0..schedule_config.day.count {
            let mut sum = 0;
            for r in 0..schedule_config.staff.count {
                if schedule[r][c] == self.column_stats_shifts[index] {
                    sum += 1;
                }
            }
            write!(self.out, "<td scope=\"col\">{}</th>", sum)?;
        }

        write!(self.out, "<tr>")?;

        Ok(())
    }

    fn write_style(&mut self) -> io::Result<()> {
        write!(
            self.out,
            "
<style>
table {{
    border-collapse: collapse;
    border: 2px solid rgb(140 140 140);
    font-family: sans-serif;
    font-size: 0.8rem;
    letter-spacing: 1px;
}}

thead,
tfoot {{
    background-color: rgb(228 240 245);
}}

th,
td {{
    border: 1px solid rgb(160 160 160);
    padding: 8px 10px;
}}

td {{
    text-align: center;
}}

tbody > tr:nth-of-type(even) {{
    background-color: rgb(237 238 242);
}}

.padding {{
    background-color: white;
    border: none;
}}
</style>"
        )?;
        Ok(())
    }
}
