use std::collections::HashMap;

/// 職員を管理する型
#[derive(Clone, Debug, Default)]
pub struct Staff {
    pub name: String,
    pub attributes: Vec<i32>,
}

/// NGを管理する型
pub type NG = (usize, usize);

/// StaffAttribute名のエイリアス
pub type StaffAttributeName = String;

/// StaffAttributeの名前からインデックスへの変換を行うための型
#[derive(Clone, Debug, Default)]
pub struct StaffAttributeNameIndexMap {
    pub names: Vec<StaffAttributeName>,
    pub name_to_index: HashMap<StaffAttributeName, usize>,
}
