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
    // pub use_seed: bool,
    pub seed: Option<u64>,
}
// use_seed挟んだほうがいいか?
// あるかないかで処理変えるべきでない？
// use_seedあった場合、use_seedがtrueでseedが未記入の場合どうするか
// エラー出すのか0とかで適当にするのか、threadrngにするのか

#[derive(Debug, serde::Deserialize)]
pub struct RawAnnealingTable {
    pub config_paths: Vec<String>,
}
// 下のAnnealingConfigとかぶる

#[derive(Debug, serde::Deserialize)]
pub struct RawResultTable {
    pub score_functions: Vec<String>,
}
