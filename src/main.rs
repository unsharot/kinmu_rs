pub mod annealing;
pub mod iofile;
pub mod score;
pub mod update;
pub mod kata;
pub mod show_hyou;
pub mod fill;

use rand::{Rng, SeedableRng, RngCore};
use rand::rngs::StdRng;
use std::io;
use std::time::Instant;

fn updatef<R: Rng>(x: &f32, rng: &mut R) -> f32{
    // x + 0.01
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

    match iofile::load_main_config(&config_path) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
        }
    }

    let ps = iofile::load_main_config(&config_path)?;

    ps.iter().for_each(|p| { let _ = sub(p); });

    Ok(())
}

fn gen_rng_from_seed(seed: usize) -> Box<dyn RngCore> {
    if seed == 0 {
        Box::new(rand::thread_rng())
    } else {
        Box::new(StdRng::seed_from_u64(seed as u64))
    }
}

fn sub(p: &str) -> io::Result<()> {
    let Ok((hp, fs, ff)) = iofile::load_config(p) else { todo!() };

    let acs: Vec<kata::AnnealingConfig> = fs.iter().map(|s| iofile::load_annealing_config(s).unwrap()).collect();

    let hst_p = &hp.hyou_st;

    let mut rng = gen_rng_from_seed(hp.seed);

    let mut model = fill::run(&ff, &hp, &mut rng);

    // let mut score: f32;
    let mut temp_score;
    for ac in acs {
        let start = Instant::now();
        rng = gen_rng_from_seed(ac.seed);
        (temp_score, model) = annealing::annealing(
            10000000000.0,
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
        println!("score: {}", temp_score);
        println!("time: {:?}", start.elapsed());
    }
    
    println!();

    let score = score::assess_score(&hp.score_props, &hp, &model);

    println!("{}", score);
    // println!("{}", score::assess_score(&hp, &model));
    show_hyou::show(&model, &hp);

    println!();

    println!("{}", score::show_score(&hp.score_props, &hp, &model));

    println!();

    Ok(())
}