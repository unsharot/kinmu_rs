pub mod test_lib;
pub mod annealing;
pub mod iofile;

pub mod kata;
// use kata::*;

// use crate::test_lib::test_lib::test_func;
use rand::Rng;
use std::io;

fn updatef(x: &f32) -> f32{
    // x + 0.01
    x + rand::thread_rng().gen::<f32>() / 100.0
}

fn evalf(x: &f32) -> f32{
    (x - 5.0) * (x - 5.0)
}

fn main() -> io::Result<()> {
    println!("Hello, world!");
    test_lib::test_lib::test_func();
    // test_func();
    test_lib::m::test_func();
    test_lib::test_func();

    let best_score: f32;
    let best_model: f32;

    (best_score, best_model) = annealing::annealing(10000.0, 0.0, 100000, updatef, evalf, 10.0, 0.0, annealing::basic_temp_func, annealing::basic_prob_func);
    
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

    ps.iter().for_each(|s| println!("{}",s));

    Ok(())
}
