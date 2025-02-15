//! 生成を行うモジュール

use ::kinmu_annealing;
use ::kinmu_core::Generator;
use ::kinmu_lib::types::{
    AnnealingConfig, Answer, FillConfig, MainConfig, Schedule, ScheduleConfig,
};
use ::kinmu_lib::{fill, score, seed, update};

use std::thread;
use std::time::Instant;

#[derive(Debug)]
pub struct GeneratorWithAnnealing;

#[allow(clippy::new_without_default)]
impl GeneratorWithAnnealing {
    pub fn new() -> Self {
        GeneratorWithAnnealing
    }
}

impl Generator<MainConfig, Vec<Answer>> for GeneratorWithAnnealing {
    fn run(&mut self, config: &MainConfig) -> anyhow::Result<Vec<Answer>> {
        generate_schedules(config)
    }
}

fn generate_schedules(config: &MainConfig) -> anyhow::Result<Vec<Answer>> {
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
) -> anyhow::Result<Answer> {
    let start = Instant::now();

    let mut hs: Vec<thread::JoinHandle<anyhow::Result<_>>> = vec![];
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
        let model = h.join().unwrap()?.clone();

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
) -> anyhow::Result<Schedule> {
    let mut model = fill::run(&mut fill_config, &schedule_config)?;

    let mut score;
    for mut ac in annealing_configs {
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = score::eval_scores(&mut ac.score_props, &schedule_config, &model);
        (_, model) = kinmu_annealing::run(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &schedule_config)?,
            |m| score::eval_scores(&mut ac.score_props, &schedule_config, m),
            ac.max_temp,
            ac.min_temp,
            kinmu_annealing::basic_temp_func,
            kinmu_annealing::basic_prob_func,
            &mut rng,
        );
    }

    Ok(model)
}
