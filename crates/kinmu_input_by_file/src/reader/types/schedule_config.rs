//! schedule_configを読み込むための構造体

use kinmu_model::Score;

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawScheduleConfig {
    pub staff: RawStaffTable,
    pub day: RawDayTable,
    pub fill: RawFillTable,
    pub annealing: RawAnnealingTable,
    pub result: RawResultTable,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawStaffTable {
    pub attributes: Vec<String>,
    pub list: Vec<RawStaffListNode>,
    pub ng_list: Vec<RawNGListNode>,
    pub count: usize,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawStaffListNode {
    pub name: String,
    pub attributes: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawNGListNode {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawDayTable {
    pub day_count: usize,
    pub buffer_count: usize,
    pub states: String,
    pub requested_schedule: Vec<String>,
    pub attributes: Vec<RawAttributeTable>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawAttributeTable {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawFillTable {
    pub function: String,
    pub seed: Option<u64>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawAnnealingTable {
    pub config_paths: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawResultTable {
    pub score_functions: Vec<RawResultScoreFunction>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawResultScoreFunction {
    pub display_name: String,
    pub scores: Vec<String>,
    pub warning: Option<RawScoreWarning>,
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize)]
pub struct RawScoreWarning {
    pub min_pass: Option<Score>,
    pub max_pass: Option<Score>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_read() {
        let config: RawScheduleConfig = toml::from_str(
            r#"
            [staff]

            attributes = [
                "AttributeA",
                "AttributeB",
                "AttributeC",
            ]

            list = [
            #             A  B   C           名前     #番号
            {attributes = [1, 1, 10], name = "職員A"}, #0
            {attributes = [0, 1,  9], name = "職員B"}, #1
            {attributes = [0, 0, 14], name = "職員C"}, #2
            ]

            ng_list = [
            {from = 0, to = 1},
            ]

            count = 3

            [day]

            day_count = 5

            buffer_count = 3

            states = "ABCDE"

            requested_schedule = [
                "NK   ",
                "KK   ",
                "KN K ",
            ]

            attributes = [{name = "DayAttribute", values = [1, 2, 1]}]

            [fill]
            function = "fill_func"
            seed = 2048

            [annealing]
            config_paths = [
                "path_a.toml",
                "path_b.toml",
            ]

            [result]
            score_functions = [
            {display_name = "sf1", scores = [
                "SP1",
            ]},
            {display_name = "sf2", scores = [
                "SP2",
                "SP3",
            ], warning = {min_pass = 0}},
            ]
            "#,
        )
        .unwrap();

        assert_eq!(
            config.staff.attributes,
            vec![
                String::from("AttributeA"),
                String::from("AttributeB"),
                String::from("AttributeC"),
            ]
        );
        assert_eq!(
            config.staff.list,
            vec![
                RawStaffListNode {
                    attributes: vec![1, 1, 10],
                    name: String::from("職員A")
                },
                RawStaffListNode {
                    attributes: vec![0, 1, 9],
                    name: String::from("職員B")
                },
                RawStaffListNode {
                    attributes: vec![0, 0, 14],
                    name: String::from("職員C")
                },
            ]
        );
        assert_eq!(config.staff.ng_list, vec![RawNGListNode { from: 0, to: 1 }]);
        assert_eq!(config.staff.count, 3);

        assert_eq!(config.day.day_count, 5);
        assert_eq!(config.day.buffer_count, 3);
        assert_eq!(config.day.states, "ABCDE");
        assert_eq!(
            config.day.requested_schedule,
            vec![
                String::from("NK   "),
                String::from("KK   "),
                String::from("KN K "),
            ]
        );
        assert_eq!(
            config.day.attributes,
            vec![RawAttributeTable {
                name: String::from("DayAttribute"),
                values: vec![1, 2, 1],
            }]
        );

        assert_eq!(config.fill.function, "fill_func");
        assert_eq!(config.fill.seed, Some(2048));

        assert_eq!(
            config.annealing.config_paths,
            vec![String::from("path_a.toml"), String::from("path_b.toml")]
        );

        assert_eq!(
            config.result.score_functions,
            vec![
                RawResultScoreFunction {
                    display_name: String::from("sf1"),
                    scores: vec![String::from("SP1")],
                    warning: None
                },
                RawResultScoreFunction {
                    display_name: String::from("sf2"),
                    scores: vec![String::from("SP2"), String::from("SP3")],
                    warning: Some(RawScoreWarning {
                        min_pass: Some(0.0),
                        max_pass: None
                    })
                }
            ]
        );
    }

    #[test]
    fn test_minimal_read() {
        let config: RawScheduleConfig = toml::from_str(
            r#"
            [staff]
            attributes = []
            list = []
            ng_list = []
            count = 0

            [day]
            day_count = 0
            buffer_count = 0
            states = ""
            requested_schedule = []
            attributes = []

            [fill]
            function = ""

            [annealing]
            config_paths = []

            [result]
            score_functions = []
            "#,
        )
        .unwrap();

        assert_eq!(config.staff.attributes, <Vec<String>>::new());
        assert_eq!(config.staff.list, <Vec<RawStaffListNode>>::new());
        assert_eq!(config.staff.ng_list, <Vec<RawNGListNode>>::new());
        assert_eq!(config.staff.count, 0);

        assert_eq!(config.day.day_count, 0);
        assert_eq!(config.day.buffer_count, 0);
        assert_eq!(config.day.states, "");
        assert_eq!(config.day.requested_schedule, <Vec<String>>::new());
        assert_eq!(config.day.attributes, <Vec<RawAttributeTable>>::new());

        assert_eq!(config.fill.function, "");
        assert_eq!(config.fill.seed, None);

        assert_eq!(config.annealing.config_paths, <Vec<String>>::new());

        assert_eq!(
            config.result.score_functions,
            <Vec<RawResultScoreFunction>>::new()
        );
    }
}
