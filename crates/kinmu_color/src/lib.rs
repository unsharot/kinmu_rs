//! 色付きで標準出力やファイル出力をする機能を提供

use std::io::{self, Write};

/// writeに用いる色の種類
#[allow(dead_code)]
pub enum Color {
    Red,
    Blue,
}

/// 色付きでテキスト出力する
pub fn write<W: Write>(out: &mut W, s: &str, color: Color, use_color: bool) -> io::Result<()> {
    if use_color {
        match color {
            Color::Red => color_print::cwrite!(out, "<red>{}</red>", s)?,
            Color::Blue => color_print::cwrite!(out, "<blue>{}</blue>", s)?,
        };
    } else {
        write!(out, "{}", s)?;
    }
    Ok(())
}
