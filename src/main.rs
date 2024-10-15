pub mod annealing;
pub mod iofile;
pub mod score;
pub mod update;
pub mod kata;
pub mod show_hyou;

// use crate::test_lib::test_lib::test_func;
use rand::Rng;
use std::io;
use std::time::Instant;

fn updatef(x: &f32) -> f32{
    // x + 0.01
    x + rand::thread_rng().gen::<f32>() / 100.0
}

fn evalf(x: &f32) -> f32{
    (x - 5.0) * (x - 5.0)
}

fn main() -> io::Result<()> {

    let best_score: f32;
    let best_model: f32;

    (best_score, best_model) = annealing::annealing(10000.0, &0.0, 100000, updatef, evalf, 10.0, 0.0, annealing::basic_temp_func, annealing::basic_prob_func);
    
    println!("{}", best_score);
    println!("{}", best_model);
    
    println!("{}",updatef(&5.0));



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

fn sub(p: &str) -> io::Result<()> {
    let Ok((hp, fs, _ff)) = iofile::load_config(p) else { todo!() };

    let acs: Vec<kata::AnnealingConfig> = fs.iter().map(|s| iofile::load_annealing_config(s).unwrap()).collect();

    let hst_p = &hp.hyou_st;

    let mut model = hp.kibou.clone(); //fillしないとだめ
    // let mut score: f32;
    let mut temp_score;
    for ac in acs {
        let start = Instant::now();
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