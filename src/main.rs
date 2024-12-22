use annealing::annealing;
use kinmu::io::{display, reader};
use kinmu::kinmu_lib::{check, fill, score, update};

use std::env;
use std::thread;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_file_path = if args.len() >= 2 {
        &args[1]
    } else {
        "config/config.yaml"
    };
    // match generate_schedules(main_file_path) {
    match generate_schedules2(main_file_path) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

fn generate_schedules(config_path: &str) -> Result<(), String> {
    let schedule_config_paths: Vec<String> =
        reader::load_main_config(config_path).map_err(|e| {
            format!(
                "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                config_path, e
            )
        })?;

    for path in schedule_config_paths {
        generate_schedule(&path)?;
    }

    Ok(())
}

fn print_check(name: &str, b: bool) {
    if !b {
        println!("[WARNING] {} CHECK FAILED", name);
    }
}

fn generate_schedule(p: &str) -> Result<(), String> {
    let (mut schedule_prop, ac_paths, mut fc) = reader::load_schedule_config(p).map_err(|e| {
        format!(
            "[エラー] 勤務表configの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
            p, e
        )
    })?;

    print_check("ALL_ABSOLUTE", check::all_absolute(&schedule_prop));

    print_check("SAFE_IAK", check::safe_iak(&schedule_prop));

    let mut model = fill::run(&mut fc, &schedule_prop)?;

    // print_check("K_I_COUNTS", check::k_i_counts(&schedule_prop, &model));

    print_check(
        "ABS_NOT_CHANGED",
        check::abs_not_changed(&schedule_prop, &model),
    );

    let mut score;
    for ac_path in ac_paths {
        let mut ac = reader::load_annealing_config(&ac_path).map_err(|e| {
            format!(
                "[エラー] 焼きなましconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                ac_path, e
            )
        })?;

        let start = Instant::now();
        let mut rng = ac.rng;
        score = score::assess_score(&mut ac.score_props, &schedule_prop, &model);
        (score, model) = annealing::annealing(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &schedule_prop)?,
            |m| score::assess_score(&mut ac.score_props, &schedule_prop, m),
            // |_| 0.0,
            ac.max_temp,
            ac.min_temp,
            annealing::basic_temp_func,
            annealing::basic_prob_func,
            &mut rng,
        );
        println!("score: {}", score);
        println!("time: {:?}", start.elapsed());
    }

    print_check("SAFE_IAK", check::safe_iak(&schedule_prop));

    print_check(
        "ABS_NOT_CHANGED",
        check::abs_not_changed(&schedule_prop, &model),
    );

    println!();

    score = score::assess_score(
        &mut schedule_prop.score_props.clone(),
        &mut schedule_prop,
        &model,
    );

    println!("{}", score);
    // println!("{}", score::assess_score(&schedule_prop, &model));
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

    println!();

    Ok(())
}

fn generate_schedules2(config_path: &str) -> Result<(), String> {
    let schedule_config_paths: Vec<String> =
        reader::load_main_config(config_path).map_err(|e| {
            format!(
                "[エラー] メインconfigの読み込みに失敗しました\n対象ファイル: {}\n理由: {}",
                config_path, e
            )
        })?;

    for path in schedule_config_paths {
        generate_schedule(&path)?;
    }

    Ok(())
}

fn generate_schedule2(p: &str) -> Result<(), String> {
    let (mut schedule_prop, ac_paths, mut fc) = reader::load_schedule_config(p).map_err(|e| {
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

    let n = 6;

    let mut hs = vec![];
    for i in 0..n {
        let schedule_prop = schedule_prop.clone();
        let annealing_configs = annealing_configs.clone();
        let mut fc = fc.clone();
        hs.push(thread::spawn(move || {
            let mut model = fill::run(&mut fc, &schedule_prop)?;

            let mut score;
            for ac in annealing_configs {
                let mut rng = ac.rng;
                score = score::assess_score(&mut ac.score_props, &schedule_prop, &model);
                (score, model) = annealing::annealing(
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
        }))
    }

    for (i, h) in hs.iter().enumerate() {
        let model = h.join().unwrap()?;

        println!("thread: {}", i);

        println!();

        let score = score::assess_score(
            &mut schedule_prop.score_props.clone(),
            &mut schedule_prop,
            &model,
        );

        println!("{}", score);
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

        println!();
    }

    Ok(())
}
