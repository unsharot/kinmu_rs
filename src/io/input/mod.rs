//! toml形式のconfigを読み込むためのモジュール
//! パスを受け取り、configを返す

use crate::kinmu_lib::types::MainConfig;

mod checker;
mod converter;
mod reader;

pub fn load_config(path: &str) -> Result<MainConfig, String> {
    let raw_main = reader::read_main_config(path).map_err(|e| {
        format!(
            "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}\nヒント: デフォルト以外のファイルを指定する場合、引数でパスを指定してください",
            path, e,
        )
    })?;
    let schedule_config_paths = raw_main.schedule_config_paths.clone();
    let mut converted_main: MainConfig = converter::convert_main_config(raw_main)?;

    for path in schedule_config_paths {
        let raw_schedule = reader::read_schedule_config(&path).map_err(|e| {
            format!(
                "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                path, e
            )
        })?;
        let annealing_config_paths = raw_schedule.annealing.config_paths.clone();
        let mut converted_schedule =
            converter::convert_schedule_config(raw_schedule).map_err(|e| {
                format!(
                    "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                    path, e
                )
            })?;

        for path in annealing_config_paths {
            let raw_annealing = reader::read_annealing_config(&path).map_err(|e| {
                format!(
                    "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                    path, e
                )
            })?;
            let converted_annealing =
                converter::convert_annealing_config(raw_annealing).map_err(|e| {
                    format!(
                    "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                    path, e
                )
                })?;
            converted_schedule
                .annealing_configs
                .push(converted_annealing);
        }

        converted_main.schedule_configs.push(converted_schedule);
    }

    checker::run(&converted_main)
        .map_err(|e| format!("[エラー] configの変換チェックに失敗しました\n理由: {}", e,))?;

    Ok(converted_main)
}
