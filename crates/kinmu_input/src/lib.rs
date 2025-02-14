//! toml形式のconfigを読み込むためのモジュール
//! パスを受け取り、configを返す

use anyhow::Context;

use ::kinmu_core::Input;
use ::kinmu_lib::types::MainConfig;

mod checker;
mod converter;
mod reader;

#[derive(Debug)]
pub struct InputByFile<'a> {
    main_config_path: &'a str,
}

impl<'a> InputByFile<'a> {
    pub fn new(main_config_path: &'a str) -> Self {
        InputByFile { main_config_path }
    }
}

impl Input<MainConfig> for InputByFile<'_> {
    fn load_config(&mut self) -> anyhow::Result<MainConfig> {
        let raw_main = reader::read_main_config(self.main_config_path).with_context(|| {
            format!(
                "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\nヒント: デフォルト以外のファイルを指定する場合、引数でパスを指定してください",
                self.main_config_path,
            )
        })?;
        let schedule_config_paths = raw_main.schedule_config_paths.clone();
        let mut converted_main: MainConfig = converter::convert_main_config(raw_main)?;

        for path in schedule_config_paths {
            let raw_schedule = reader::read_schedule_config(&path).with_context(|| {
                format!(
                    "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}",
                    path,
                )
            })?;
            let annealing_config_paths = raw_schedule.annealing.config_paths.clone();
            let mut converted_schedule = converter::convert_schedule_config(raw_schedule)
                .with_context(|| {
                    format!(
                        "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}",
                        path,
                    )
                })?;

            for path in annealing_config_paths {
                let raw_annealing = reader::read_annealing_config(&path).with_context(|| {
                    format!(
                        "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}",
                        path,
                    )
                })?;
                let converted_annealing = converter::convert_annealing_config(raw_annealing)
                    .with_context(|| {
                        anyhow::anyhow!(
                            "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}",
                            path,
                        )
                    })?;
                converted_schedule
                    .annealing_configs
                    .push(converted_annealing);
            }

            converted_main.schedule_configs.push(converted_schedule);
        }

        checker::run(&converted_main).context("[エラー] configの変換チェックに失敗しました")?;

        Ok(converted_main)
    }
}
