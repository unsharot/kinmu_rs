//! 生成を行うモジュール

use crate::io::input;
use crate::kinmu_lib::types::{AnnealingConfig, Answer, FillConfig, Schedule, ScheduleConfig};
use crate::kinmu_lib::{fill, score, seed, update};
use ::annealing::annealing;

use std::thread;
use std::time::Instant;

pub fn run(main_config_path: &str) -> Result<Vec<Answer>, String> {
    generate_schedules(main_config_path)
}

fn generate_schedules(main_config_path: &str) -> Result<Vec<Answer>, String> {
    let main_config = input::load_main_config(main_config_path).map_err(|e| {
        format!(
            "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}\nヒント: デフォルト以外のファイルを指定する場合、引数でパスを指定してください",
            main_config_path, e,
        )
    })?;

    let schedule_config_paths = main_config.schedule_config_paths;
    let thread_count = main_config.thread_count.unwrap_or(1);

    let mut answers = Vec::new();
    for path in schedule_config_paths {
        answers.push(generate_schedule(&path, thread_count)?);
    }

    Ok(answers)
}

fn generate_schedule(p: &str, thread_count: u32) -> Result<Answer, String> {
    let (schedule_config, ac_paths, fc) = input::load_schedule_config(p).map_err(|e| {
        format!(
            "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
            p, e
        )
    })?;

    let mut annealing_configs = vec![];
    for ac_path in ac_paths {
        annealing_configs.push(input::load_annealing_config(&ac_path).map_err(|e| {
            format!(
                "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                ac_path, e
            )
        })?);
    }

    let start = Instant::now();

    let mut hs: Vec<thread::JoinHandle<Result<_, String>>> = vec![];
    for _ in 0..thread_count {
        let schedule_config = schedule_config.clone();
        let annealing_configs = annealing_configs.clone();
        let fc = fc.clone();
        hs.push(thread::spawn(move || {
            annealing(schedule_config, fc, annealing_configs)
        }))
    }

    let mut models = Vec::new();
    for h in hs.into_iter() {
        let model = h.join().unwrap().clone()?;

        models.push(model);
    }

    Ok(Answer {
        models,
        schedule_config,
        total_time: start.elapsed(),
    })
}

fn annealing(
    schedule_config: ScheduleConfig,
    mut fc: FillConfig,
    annealing_configs: Vec<AnnealingConfig>,
) -> Result<Schedule, String> {
    let mut model = fill::run(&mut fc, &schedule_config)?;

    let mut score;
    for mut ac in annealing_configs {
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = score::assess_score(&mut ac.score_props, &schedule_config, &model);
        (_, model) = annealing::run(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &schedule_config)?,
            |m| score::assess_score(&mut ac.score_props, &schedule_config, m),
            ac.max_temp,
            ac.min_temp,
            annealing::basic_temp_func,
            annealing::basic_prob_func,
            &mut rng,
        );
    }

    Ok(model)
}
