//! schedule_configを読み込むための構造体

use kinmu_model::Score;

#[derive(Debug, serde::Deserialize)]
pub struct RawScheduleConfig {
    pub staff: RawStaffTable,
    pub day: RawDayTable,
    pub fill: RawFillTable,
    pub annealing: RawAnnealingTable,
    pub result: RawResultTable,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawStaffTable {
    pub attributes: Vec<String>,
    pub list: Vec<RawStaffListNode>,
    pub ng_list: Vec<RawNGListNode>,
    pub count: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawStaffListNode {
    pub name: String,
    pub attributes: Vec<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawNGListNode {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawDayTable {
    pub day_count: usize,
    pub buffer_count: usize,
    pub states: String,
    pub requested_schedule: Vec<String>,
    pub attributes: Vec<RawAttributeTable>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawAttributeTable {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawFillTable {
    pub function: String,
    pub seed: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingTable {
    pub config_paths: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawResultTable {
    pub score_functions: Vec<RawResultScoreFunction>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawResultScoreFunction {
    pub display_name: String,
    pub scores: Vec<String>,
    pub warning: Option<RawScoreWarning>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawScoreWarning {
    pub min_pass: Option<Score>,
    pub max_pass: Option<Score>,
}
