//! 結果をユーザーに出力するモジュール

mod text;

use ::kinmu_core::Output;
use ::kinmu_lib::types::Answer;

use std::io;

pub struct OutputText<'a, W: io::Write> {
    pub out: &'a mut W,
    pub use_color: bool,
}

impl<W: io::Write> Output<Answer> for OutputText<'_, W> {
    fn run(&mut self, answer: &Answer) -> anyhow::Result<()> {
        text::write_answer(&mut self.out, answer, self.use_color)
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
