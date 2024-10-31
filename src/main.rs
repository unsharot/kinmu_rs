use annealing::annealing;
use kinmu::kinmu_lib::{score, update, fill, check};
use kinmu::io::{reader, display};
use kinmu::seed;

use std::time::Instant;

fn main() -> Result<(), String> {

    let config_path = "config/config.yaml".to_string();
    
    let schedule_config_paths: Vec<String> = reader::load_main_config(&config_path)?;
    
    for path in schedule_config_paths {
        sub(&path)?;
    }


    Ok(())
}

fn print_check(name: &str, b: bool) {
    if !b {
        println!("[WARNING] {} CHECK FAILED", name);
    }
}

fn sub(p: &str) -> Result<(), String> {
    let (schedule_prop, ac_paths, fc) = reader::load_config(p)?;

    print_check("ALL_ABSOLUTE", check::all_absolute(&schedule_prop));

    print_check("SAFE_IAK", check::safe_iak(&schedule_prop));

    let mut model = fill::run(&fc, &schedule_prop);

    print_check("K_I_COUNTS", check::k_i_counts(&schedule_prop, &model));

    print_check("ABS_NOT_CHANGED", check::abs_not_changed(&schedule_prop, &model));

    let mut score;
    for ac_path in ac_paths {
        let ac = reader::load_annealing_config(&ac_path)?;

        let start = Instant::now();
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = score::assess_score(&ac.score_props, &schedule_prop, &model);
        (score, model) = annealing::annealing(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &schedule_prop),
            |m| score::assess_score(&ac.score_props, &schedule_prop, m),
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

    print_check("ABS_NOT_CHANGED", check::abs_not_changed(&schedule_prop, &model));
    
    println!();

    score = score::assess_score(&schedule_prop.score_props, &schedule_prop, &model);

    println!("{}", score);
    // println!("{}", score::assess_score(&schedule_prop, &model));
    display::print_schedule(&schedule_prop, &model);

    println!();

    println!("{}", score::show_score(&schedule_prop.score_props, &schedule_prop, &model));

    println!();

    Ok(())
}