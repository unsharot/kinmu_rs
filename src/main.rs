use annealing::annealing;
use kinmu::io::{display, reader};
use kinmu::kinmu_lib::types::{AnnealingConfig, FillConfig, Schedule, ScheduleProp};
use kinmu::kinmu_lib::{fill, score, seed, update};

use std::env;
use std::thread;
use std::time::Instant;

const DEFALUT_MAIN_CONFIG_PATH: &str = "config/config.yaml";

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_file_path = if args.len() >= 2 {
        &args[1]
    } else {
        DEFALUT_MAIN_CONFIG_PATH
    };
    match generate_schedules(main_file_path) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn generate_schedules(main_config_path: &str) -> Result<(), String> {
    let main_config = reader::load_main_config(main_config_path).map_err(|e| {
        format!(
            "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}\nヒント: {}以外のファイルを指定する場合、引数でパスを指定してください",
            main_config_path, e, DEFALUT_MAIN_CONFIG_PATH
        )
    })?;

    let schedule_config_paths = main_config.schedule_config_paths;
    let thread_count = main_config.thread_count.unwrap_or(1);

    for path in schedule_config_paths {
        generate_schedule(&path, thread_count)?;
    }

    Ok(())
}

fn generate_schedule(p: &str, thread_count: usize) -> Result<(), String> {
    let (mut schedule_prop, ac_paths, fc) = reader::load_schedule_config(p).map_err(|e| {
        format!(
            "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
            p, e
        )
    })?;

    let mut annealing_configs = vec![];
    for ac_path in ac_paths {
        annealing_configs.push(reader::load_annealing_config(&ac_path).map_err(|e| {
            format!(
                "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                ac_path, e
            )
        })?);
    }

    let start = Instant::now();

    let mut hs: Vec<thread::JoinHandle<Result<_, String>>> = vec![];
    for _ in 0..thread_count {
        let schedule_prop = schedule_prop.clone();
        let annealing_configs = annealing_configs.clone();
        let fc = fc.clone();
        hs.push(thread::spawn(move || {
            annealing(schedule_prop, fc, annealing_configs)
        }))
    }

    for (i, h) in hs.into_iter().enumerate() {
        let model = h.join().unwrap().clone()?;

        println!("thread: {}", i + 1);

        print_model(&mut schedule_prop, &model);

        println!();
    }

    println!();

    println!("total time: {:?}", start.elapsed());

    Ok(())
}

fn annealing(
    schedule_prop: ScheduleProp,
    mut fc: FillConfig,
    annealing_configs: Vec<AnnealingConfig>,
) -> Result<Schedule, String> {
    let mut model = fill::run(&mut fc, &schedule_prop)?;

    let mut score;
    for mut ac in annealing_configs {
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = score::assess_score(&mut ac.score_props, &schedule_prop, &model);
        (_, model) = annealing::run(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &schedule_prop)?,
            |m| score::assess_score(&mut ac.score_props, &schedule_prop, m),
            ac.max_temp,
            ac.min_temp,
            annealing::basic_temp_func,
            annealing::basic_prob_func,
            &mut rng,
        );
    }

    Ok(model)
}

fn print_model(mut schedule_prop: &mut ScheduleProp, model: &Schedule) {
    let score = score::assess_score(
        &mut schedule_prop.score_props.clone(),
        &mut schedule_prop,
        &model,
    );

    println!("score: {}", score);
    display::print_schedule(&schedule_prop, &model);

    println!();

    println!(
        "{}",
        score::show_score(
            &mut schedule_prop.score_props.clone(),
            &mut schedule_prop,
            &model
        )
    );
}
