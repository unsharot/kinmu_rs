use super::core::Output;
use super::lib::{DayState, Shift, ShiftState, StdScoreProp};
use super::model::Answer;
use super::output_html::OutputHTML;
use super::output_text::OutputText;

use std::io;

pub enum OutputTextOrHTML<'a, W: io::Write, S> {
    OutputText(OutputText<'a, W, S>),
    OutputHTML(OutputHTML<'a, W, S>),
}

impl<'a, W: io::Write, S> OutputTextOrHTML<'a, W, S> {
    pub fn new(
        use_html: bool,
        out: &'a mut W,
        use_color: bool,
        row_stats_shifts: Vec<S>,
        column_stats_shifts: Vec<S>,
    ) -> Self {
        if use_html {
            Self::OutputHTML(OutputHTML::new(out, row_stats_shifts, column_stats_shifts))
        } else {
            Self::OutputText(OutputText::new(
                out,
                use_color,
                row_stats_shifts,
                column_stats_shifts,
            ))
        }
    }
}

impl<W: io::Write> Output<Vec<Answer<StdScoreProp, Shift, ShiftState, DayState>>>
    for OutputTextOrHTML<'_, W, Shift>
{
    fn run(
        &mut self,
        answer: &Vec<Answer<StdScoreProp, Shift, ShiftState, DayState>>,
    ) -> anyhow::Result<()> {
        match self {
            Self::OutputText(o) => o.run(answer),
            Self::OutputHTML(o) => o.run(answer),
        }
    }
}
