//! 結果をユーザーに出力するモジュール

mod text;

use ::kinmu_core::Output;
use ::kinmu_lib::types::Answer;

use std::io;

#[derive(Debug)]
pub struct OutputText<'a, W: io::Write> {
    out: &'a mut W,
    use_color: bool,
}

impl<'a, W: io::Write> OutputText<'a, W> {
    pub fn new(out: &'a mut W, use_color: bool) -> Self {
        OutputText { out, use_color }
    }
}

impl<W: io::Write> Output<Vec<Answer>> for OutputText<'_, W> {
    fn run(&mut self, answers: &Vec<Answer>) -> anyhow::Result<()> {
        for answer in answers {
            text::write_answer(&mut self.out, answer, self.use_color)
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        }
        Ok(())
    }
}
