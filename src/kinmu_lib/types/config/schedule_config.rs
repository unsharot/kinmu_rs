//! schedule_configから生成できる構造体を定義

use crate::kinmu_lib::types::{
    DayAttributeName, Days, NGList, Schedule, ScheduleState, ScoreProp, Staff, StaffAttributeName,
    StaffAttributeNameIndexMap,
};

use std::collections::HashMap;

#[derive(Clone)]
pub struct FillConfig {
    pub name: String,
    pub seed: Option<u64>,
}

/// 勤務表ごとの設定
#[derive(Clone)]
pub struct ScheduleProp {
    pub staff_list: Vec<Staff>,
    pub ng_list: NGList,
    pub staff_count: usize,
    pub day_count: usize,
    pub days: Days,
    pub buffer: usize,
    pub request: Schedule,
    pub schedule_st: ScheduleState,
    pub day_attributes: HashMap<DayAttributeName, Vec<i32>>,
    pub staff_attribute_map: StaffAttributeNameIndexMap,
    pub score_props: Vec<ScoreProp>, // 結果表示のためのスコア
}

impl ScheduleProp {
    pub fn get_attribute(&self, staff: usize, attribute: &StaffAttributeName) -> i32 {
        let att_index = self
            .staff_attribute_map
            .name_to_index
            .get(attribute)
            .unwrap();
        self.staff_list[staff].attributes[*att_index]
    }
}
