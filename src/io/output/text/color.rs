//! 色付きで出力するモジュール

use color_print;
use std::io::{self, Write};

#[allow(dead_code)]
pub enum Color {
    Red,
    Blue,
}

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
