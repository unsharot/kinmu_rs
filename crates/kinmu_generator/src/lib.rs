//! 焼きなまし法を用いた生成器を提供
//! また、ここで要求するtraitを定義

mod seed;

use ::kinmu_annealing;
use ::kinmu_core::Generator;
use ::kinmu_model::{
    eval_scores_mut, AnnealingConfig, Answer, FillConfig, MainConfig, Schedule, ScheduleConfig,
    ScorePropTrait,
};

use std::thread;
use std::time::Instant;

use rand::Rng;

/// 焼きなまし法を用いた生成器
/// 初めに表を埋めるための型Fと更新のための型Uを保持
#[derive(Debug)]
pub struct GeneratorWithAnnealing<F, U> {
    fill: F,
    update: U,
}

impl<F, U> GeneratorWithAnnealing<F, U> {
    /// コンストラクタ
    /// 初めに表を埋めるための型Fと更新のための型Uを要求
    pub fn new(fill: F, update: U) -> Self {
        GeneratorWithAnnealing { fill, update }
    }
}

/// 生成器の実装
/// F, UにはFill, Updateを要求
/// また、スレッドを分けるため各型にClone + std::marker::Send + 'staticを要求
impl<SP, S, SS, DS, F, U> Generator<MainConfig<SP, S, SS, DS>, Vec<Answer<SP, S, SS, DS>>>
    for GeneratorWithAnnealing<F, U>
where
    SP: Clone + std::marker::Send + 'static + ScorePropTrait<S, SS, DS>,
    S: Clone + std::marker::Send + 'static,
    SS: Clone + std::marker::Send + 'static,
    DS: Clone + std::marker::Send + 'static,
    F: Fill<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
    U: Update<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
{
    fn run(
        &mut self,
        config: &MainConfig<SP, S, SS, DS>,
    ) -> anyhow::Result<Vec<Answer<SP, S, SS, DS>>> {
        generate_schedules(config, &self.fill, &self.update)
    }
}

/// 勤務表をMainConfigで指定した回数ループして生成
fn generate_schedules<SP, S, SS, DS, F, U>(
    config: &MainConfig<SP, S, SS, DS>,
    fill: &F,
    update: &U,
) -> anyhow::Result<Vec<Answer<SP, S, SS, DS>>>
where
    SP: Clone + std::marker::Send + 'static + ScorePropTrait<S, SS, DS>,
    S: Clone + std::marker::Send + 'static,
    SS: Clone + std::marker::Send + 'static,
    DS: Clone + std::marker::Send + 'static,
    F: Fill<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
    U: Update<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
{
    let thread_count = config.thread_count.unwrap_or(1);

    let mut answers = Vec::new();
    for schedule_config in &config.schedule_configs {
        answers.push(generate_schedule(
            schedule_config,
            thread_count,
            fill,
            update,
        )?);
    }

    Ok(answers)
}

/// 勤務表をマルチスレッドで複数生成
fn generate_schedule<SP, S, SS, DS, F, U>(
    schedule_config: &ScheduleConfig<SP, S, SS, DS>,
    thread_count: u32,
    fill: &F,
    update: &U,
) -> anyhow::Result<Answer<SP, S, SS, DS>>
where
    SP: Clone + std::marker::Send + 'static + ScorePropTrait<S, SS, DS>,
    S: Clone + std::marker::Send + 'static,
    SS: Clone + std::marker::Send + 'static,
    DS: Clone + std::marker::Send + 'static,
    F: Fill<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
    U: Update<SP, S, SS, DS> + Clone + std::marker::Send + 'static,
{
    let start = Instant::now();

    let mut hs: Vec<thread::JoinHandle<anyhow::Result<_>>> = vec![];
    for _ in 0..thread_count {
        let schedule_config = schedule_config.clone();
        let annealing_configs = schedule_config.annealing_configs.clone();
        let fill_config = schedule_config.fill.clone();
        let fill = fill.clone();
        let update = update.clone();
        hs.push(thread::spawn(move || {
            annealing(
                schedule_config,
                fill_config,
                annealing_configs,
                fill,
                update,
            )
        }))
    }

    let mut models = Vec::new();
    for h in hs.into_iter() {
        models.push(h.join().unwrap()?);
    }

    Ok(Answer {
        models,
        schedule_config: schedule_config.clone(),
        total_time: start.elapsed(),
    })
}

/// 焼きなましを実行する
fn annealing<SP, S, SS, DS, F, U>(
    schedule_config: ScheduleConfig<SP, S, SS, DS>,
    fill_config: FillConfig,
    annealing_configs: Vec<AnnealingConfig<SP>>,
    fill: F,
    update: U,
) -> anyhow::Result<Schedule<S>>
where
    SP: ScorePropTrait<S, SS, DS>,
    S: Clone,
    F: Fill<SP, S, SS, DS>,
    U: Update<SP, S, SS, DS>,
{
    let mut model = fill.run(
        &fill_config.name,
        &schedule_config,
        &mut seed::gen_rng_from_seed(fill_config.seed),
    )?;

    let mut score;
    for mut ac in annealing_configs {
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = eval_scores_mut(&mut ac.score_props, &schedule_config, &model);
        (_, model) = kinmu_annealing::run(
            score,
            &model,
            ac.step,
            update.generate(&ac.update_func, &schedule_config)?,
            |m| eval_scores_mut(&mut ac.score_props, &schedule_config, m),
            ac.max_temp,
            ac.min_temp,
            kinmu_annealing::basic_temp_func,
            kinmu_annealing::basic_prob_func,
            &mut rng,
        );
    }

    Ok(model)
}

/// GeneratorWithAnnealingで用いるFillの共通のふるまい
pub trait Fill<SP, S, SS, DS> {
    /// 名前とScheduleConfig, Rngから埋めた表を出力
    fn run<R: Rng>(
        &self,
        name: &str,
        schedule_config: &ScheduleConfig<SP, S, SS, DS>,
        rng: &mut R,
    ) -> anyhow::Result<Schedule<S>>;
}

/// GeneratorWithAnnealingで用いるUpdateの共通のふるまい
#[allow(clippy::type_complexity)]
pub trait Update<SP, S, SS, DS> {
    /// 名前とScheduleConfigからクロージャーを生成
    fn generate<'a, R: Rng>(
        &self,
        name: &str,
        schedule_config: &'a ScheduleConfig<SP, S, SS, DS>,
    ) -> anyhow::Result<Box<dyn FnMut(&Schedule<S>, &mut R) -> Schedule<S> + 'a>>;
}
