//! 生成を行うモジュール

use crate::kinmu_lib::types::{
    AnnealingConfig, Answer, FillConfig, MainConfig, Schedule, ScheduleConfig,
};
use crate::kinmu_lib::{fill, score, seed, update};
use ::annealing;

use std::thread;
use std::time::Instant;

pub fn run(config: &MainConfig) -> Result<Vec<Answer>, String> {
    generate_schedules(config)
}

fn generate_schedules(config: &MainConfig) -> Result<Vec<Answer>, String> {
    let thread_count = config.thread_count.unwrap_or(1);

    let mut answers = Vec::new();
    for schedule_config in &config.schedule_configs {
        answers.push(generate_schedule(schedule_config, thread_count)?);
    }

    Ok(answers)
}

fn generate_schedule(
    schedule_config: &ScheduleConfig,
    thread_count: u32,
) -> Result<Answer, String> {
    let start = Instant::now();

    let mut hs: Vec<thread::JoinHandle<Result<_, String>>> = vec![];
    for _ in 0..thread_count {
        let schedule_config = schedule_config.clone();
        let annealing_configs = schedule_config.annealing_configs.clone();
        let fill_config = schedule_config.fill.clone();
        hs.push(thread::spawn(move || {
            annealing(schedule_config, fill_config, annealing_configs)
        }))
    }

    let mut models = Vec::new();
    for h in hs.into_iter() {
        let model = h.join().unwrap().clone()?;

        models.push(model);
    }

    Ok(Answer {
        models,
        schedule_config: schedule_config.clone(),
        total_time: start.elapsed(),
    })
}

fn annealing(
    schedule_config: ScheduleConfig,
    mut fill_config: FillConfig,
    annealing_configs: Vec<AnnealingConfig>,
) -> Result<Schedule, String> {
    let mut model = fill::run(&mut fill_config, &schedule_config)?;

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
