//! 職員に関わる型の宣言

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Staff {
    pub name: String,
    pub attributes: Vec<i32>,
}

pub type NG = (usize, usize);

pub type NGList = Vec<NG>;

pub type StaffAttributeName = String;

#[derive(Clone, Debug)]
pub struct StaffAttributeNameIndexMap {
    pub names: Vec<StaffAttributeName>,
    pub name_to_index: HashMap<StaffAttributeName, usize>,
}
