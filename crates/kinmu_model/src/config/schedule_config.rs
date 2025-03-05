use super::super::{
    Schedule, ScheduleState, Score, Staff, StaffAttributeName, StaffAttributeNameIndexMap, NG,
};

use std::collections::HashMap;

use super::AnnealingConfig;

/// Fillに関するConfig
#[derive(Clone, Debug, Default)]
pub struct FillConfig {
    pub name: String,
    pub seed: Option<u64>,
}

/// 結果の出力に関するConfig
#[derive(Clone, Debug, Default)]
pub struct ResultConfig<SP> {
    pub score_functions: Vec<ScoreFunction<SP>>,
}

/// ResultConfigで用いるスコアのプロパティ
#[derive(Clone, Debug, Default)]
pub struct ScoreFunction<SP> {
    pub display_name: String,
    pub scores: Vec<SP>,
    pub filter: Option<ScoreFilter>,
}

/// 結果で用いるスコアのフィルター
#[derive(Clone, Debug, Default)]
pub struct ScoreFilter {
    pub low_pass: Option<Score>,
    pub high_pass: Option<Score>,
}

/// スタッフに関するConfig
#[derive(Clone, Debug, Default)]
pub struct StaffConfig {
    pub attribute_map: StaffAttributeNameIndexMap,
    pub list: Vec<Staff>,
    pub ng_list: Vec<NG>,
    pub count: usize,
}

/// DayAttribute名のエイリアス
pub type DayAttributeName = String;

/// 日あたりのConfig
#[derive(Clone, Debug, Default)]
pub struct DayConfig<S, SS, DS> {
    pub count: usize,
    pub buffer_count: usize,
    pub days: Vec<DS>,
    pub requested_schedule: Schedule<S>,
    pub schedule_states: ScheduleState<SS>,
    pub attributes: HashMap<DayAttributeName, Vec<i32>>,
}

/// 勤務表ごとのConfig
#[derive(Clone, Debug, Default)]
pub struct ScheduleConfig<SP, S, SS, DS> {
    pub staff: StaffConfig,
    pub day: DayConfig<S, SS, DS>,
    pub fill: FillConfig,
    pub annealing_configs: Vec<AnnealingConfig<SP>>,
    pub result: ResultConfig<SP>,
}

impl<SP, S, SS, DS> ScheduleConfig<SP, S, SS, DS> {
    /// 指定したスタッフの指定したattributeを取得する
    pub fn get_attribute(&self, staff: usize, attribute: &StaffAttributeName) -> i32 {
        let att_index = self
            .staff
            .attribute_map
            .name_to_index
            .get(attribute)
            .unwrap();
        self.staff.list[staff].attributes[*att_index]
    }
}
