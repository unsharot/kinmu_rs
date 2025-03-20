//! toml形式のconfigファイルを読み込む入力器を提供
//! パスを受け取り、MainConfigを返す
//! また、ここで要求するtraitを定義

use anyhow::Context;

use kinmu_core::Input;
use kinmu_model::MainConfig;

mod checker;
mod converter;
mod reader;

// トレイトを外部に公開
pub use checker::Check;
pub use converter::{FromConfig, MapState};

/// toml形式のconfigファイルを読み込む入力器
/// main_configのパスを保持
#[derive(Debug)]
pub struct InputByFile<'a> {
    main_config_path: &'a str,
}

impl<'a> InputByFile<'a> {
    /// コンストラクタ
    /// main_configのパスを要求
    pub fn new(main_config_path: &'a str) -> Self {
        InputByFile { main_config_path }
    }
}

/// 入力器の実装
/// ScorePropにあたるSPにはCloneとCheckを要求
/// ShiftにあたるSにはMapStateを要求
/// SP, S, DayStateにあたるDSにはそれぞれFromConfigを要求
impl<SP, S, SS, DS> Input<MainConfig<SP, S, SS, DS>> for InputByFile<'_>
where
    SP: FromConfig + Clone + Check<SP, S, SS, DS>,
    S: FromConfig + MapState<SS>,
    DS: FromConfig,
{
    fn load_config(&mut self) -> anyhow::Result<MainConfig<SP, S, SS, DS>> {
        let raw_main = reader::read_main_config(self.main_config_path).with_context(|| {
            format!(
                "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\nヒント: デフォルト以外のファイルを指定する場合、引数でパスを指定してください",
                self.main_config_path,
            )
        })?;
        let schedule_config_paths = raw_main.schedule_config_paths.clone();
        let mut converted_main: MainConfig<SP, S, SS, DS> =
            converter::convert_main_config(raw_main)?;

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
