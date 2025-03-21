//! 生のtoml形式のconfig型を内部で使用するMainConfig型に変換するモジュール

mod util;

mod annealing_config;
mod main_config;
mod schedule_config;

pub(super) use self::annealing_config::*;
pub(super) use self::main_config::*;
pub use self::schedule_config::MapState;
pub(super) use self::schedule_config::*;
pub use self::util::parser::FromConfig;
