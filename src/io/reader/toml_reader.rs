///! tomlファイルを読み込むモジュール
use serde;
use std::fs;
use toml;

type FilePath = String;

#[derive(Debug, serde::Deserialize)]
pub struct MainConfig {
    pub schedule_config_paths: Vec<FilePath>,
    pub thread_count: Option<usize>,
}

pub fn read_main_config(path: &str) -> Result<MainConfig, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let config: MainConfig = toml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}

#[derive(Debug, serde::Deserialize)]
pub struct StaffListNode {
    pub name: String,
    pub attributes: Vec<isize>,
}

#[derive(Debug, serde::Deserialize)]
pub struct NGListNode {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct StaffC {
    pub attributes: Vec<String>,
    pub list: Vec<StaffListNode>,
    pub ng_list: Vec<NGListNode>,
    pub count: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct DayC {
    pub day_count: usize,
    pub states: String,
    pub buffer_count: usize,
    pub requested_schedule: Vec<String>,
    pub attributes: Vec<Att>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Att {
    pub name: String,
    pub values: Vec<isize>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Fill {
    pub function: String,
    pub seed: usize,
}

#[derive(Debug, serde::Deserialize)]
pub struct Annealing {
    pub config_paths: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ResultC {
    pub score_functions: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ScheduleConfig {
    pub staff: StaffC,
    pub day: DayC,
    pub fill: Fill,
    pub annealing: Annealing,
    pub result: ResultC,
}

pub fn read_schedule_config(path: &str) -> Result<ScheduleConfig, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let config: ScheduleConfig = toml::from_str(&contents).map_err(|e| e.to_string())?;

    Ok(config)
}

#[derive(Debug, serde::Deserialize)]
pub struct AnnealingC {
    pub step_count: usize,
    pub seed: usize,
    pub score_functions: Vec<String>,
    pub update_function: String,
    pub temp: Temp,
}

#[derive(Debug, serde::Deserialize)]
pub struct Temp {
    pub max: f32,
    pub min: f32,
}

pub fn read_annealing_config(path: &str) -> Result<AnnealingC, String> {
    let contents = fs::read_to_string(path).map_err(|e| e.to_string())?;

    let config: AnnealingC = toml::from_str(&contents).map_err(|e| e.to_string())?;

    Ok(config)
}
