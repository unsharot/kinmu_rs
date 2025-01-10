//! schedule_configから生成できる構造体を定義

use crate::kinmu_lib::types::{
    DayAttributeName, DayState, Schedule, ScheduleState, ScoreProp, Staff, StaffAttributeName,
    StaffAttributeNameIndexMap, NG,
};

use std::collections::HashMap;

use super::AnnealingConfig;

#[derive(Clone, Debug, Default)]
pub struct FillConfig {
    pub name: String,
    pub seed: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct ResultConfig {
    pub score_props: Vec<ScoreProp>,
}

#[derive(Clone, Debug, Default)]
pub struct StaffConfig {
    pub attribute_map: StaffAttributeNameIndexMap,
    pub list: Vec<Staff>,
    pub ng_list: Vec<NG>,
    pub count: usize,
}

#[derive(Clone, Debug, Default)]
pub struct DayConfig {
    pub count: usize,
    pub buffer_count: usize,
    pub days: Vec<DayState>,
    pub requested_schedule: Schedule,
    pub schedule_states: ScheduleState,
    pub attributes: HashMap<DayAttributeName, Vec<i32>>,
}

#[derive(Clone, Debug, Default)]
pub struct ScheduleConfig {
    pub staff: StaffConfig,
    pub day: DayConfig,
    pub fill: FillConfig,
    pub annealing_configs: Vec<AnnealingConfig>,
    pub result: ResultConfig,
}

impl ScheduleConfig {
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
