use ::kinmu::generator;
use ::kinmu::io::input;
use ::kinmu::io::output;

use std::env;

const DEFALUT_MAIN_CONFIG_PATH: &str = "example/main_config.toml";

fn main() {
    let args: Vec<String> = env::args().collect();
    let main_file_path = if args.len() >= 2 {
        &args[1]
    } else {
        DEFALUT_MAIN_CONFIG_PATH
    };

    match run(main_file_path) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}

fn run(main_file_path: &str) -> Result<(), String> {
    let config = input::load_config(main_file_path)?;

    let answers = generator::run(&config)?;

    for ans in answers {
        output::run(ans);
    }

    Ok(())
}
