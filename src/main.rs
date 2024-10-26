use annealing::annealing;
use kinmu::kinmu_lib::{score, update, types, fill, check};
use kinmu::io::{reader, display};
use kinmu::seed;

use rand::Rng;
use std::io;
use std::time::Instant;

fn updatef<R: Rng>(x: &f32, rng: &mut R) -> f32{
    x + rng.gen::<f32>() / 100.0
}

fn evalf(x: &f32) -> f32{
    (x - 5.0) * (x - 5.0)
}

fn main() -> io::Result<()> {

    let best_score: f32;
    let best_model: f32;

    (best_score, best_model) = annealing::annealing(10000.0, &0.0, 100000, updatef, evalf, 10.0, 0.0, annealing::basic_temp_func, annealing::basic_prob_func,
        &mut rand::thread_rng(),
        // &mut StdRng::seed_from_u64(0),
    );
    
    println!("{}", best_score);
    println!("{}", best_model);



    let config_path = "config/config.yaml".to_string();

    match reader::load_main_config(&config_path) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
        }
    }

    let ps = reader::load_main_config(&config_path)?;

    ps.iter().for_each(|p| { let _ = sub(p); });

    Ok(())
}

fn print_check(name: &str, b: bool) {
    if b {
        println!("{} CHECK PASSED", name);
    } else {
        println!("{} CHECK FAILED", name);
    }
}

fn sub(p: &str) -> io::Result<()> {
    let Ok((hp, fs, fc)) = reader::load_config(p) else { todo!() };

    print_check("ALL_ABSOLUTE", check::all_absolute(&hp));

    print_check("SAFE_IAK", check::safe_iak(&hp));

    let acs: Vec<types::AnnealingConfig> = fs.iter().map(|s| reader::load_annealing_config(s).unwrap()).collect();

    let hst_p = &hp.hyou_st;


    let mut model = fill::run(&fc, &hp);

    print_check("K_I_COUNTS", check::k_i_counts(&hp, &model));

    print_check("ABS_NOT_CHANGED", check::abs_not_changed(&hp, &model));

    let mut score;
    for ac in acs {
        let start = Instant::now();
        let mut rng = seed::gen_rng_from_seed(ac.seed);
        score = score::assess_score(&ac.score_props, &hp, &model);
        (score, model) = annealing::annealing(
            score,
            &model,
            ac.step,
            update::gen_update_func(&ac.update_func, &hp, hst_p),
            |m| score::assess_score(&ac.score_props, &hp, m),
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

    print_check("SAFE_IAK", check::safe_iak(&hp));

    print_check("ABS_NOT_CHANGED", check::abs_not_changed(&hp, &model));
    
    println!();

    score = score::assess_score(&hp.score_props, &hp, &model);

    println!("{}", score);
    // println!("{}", score::assess_score(&hp, &model));
    display::print_hyou(&hp, &model);

    println!();

    println!("{}", score::show_score(&hp.score_props, &hp, &model));

    println!();

    Ok(())
}