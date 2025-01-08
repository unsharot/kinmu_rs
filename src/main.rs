use ::kinmu::generator;
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
    match generator::run(main_file_path) {
        Ok(answers) => {
            for ans in answers {
                output::run(ans);
            }
        }
        Err(e) => println!("{}", e),
    }
}
