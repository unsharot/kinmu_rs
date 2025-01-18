use ::kinmu::generator;
use ::kinmu::io::input;
use ::kinmu::io::output;

use getopts::Options;
use std::env;

const DEFALUT_MAIN_CONFIG_PATH: &str = "example/main_config.toml";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} PATH [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // オプションを登録
    let mut opts = Options::new();
    opts.optopt(
        "l",
        "load",
        "select config path and run 設定ファイルを指定して実行",
        "PATH",
    );
    opts.optflag("h", "help", "show help ヘルプを表示");
    opts.optflag("v", "version", "show version バージョンを表示");

    // オプションの読み込み
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            color_print::cprintln!("<red>[エラー]</red> オプションが不正です");
            color_print::cprintln!("理由: {}", f.to_string());
            print_usage(&program, opts);
            return;
        }
    };

    // ヘルプを表示
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    };

    // バージョンを表示
    if matches.opt_present("v") {
        println!("kinmu {}", VERSION);
        println!("License: {}", LICENSE);
        println!("Repository: {}", REPOSITORY);
        return;
    };

    // loadオプションの読み込み
    let load_option = matches.opt_str("l");
    let main_file_path = match load_option {
        Some(ref x) => x,
        None => DEFALUT_MAIN_CONFIG_PATH,
    };

    // 実行
    match run(main_file_path) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    };
}

fn run(main_file_path: &str) -> Result<(), String> {
    let config = input::load_config(main_file_path)?;

    let answers = generator::run(&config)?;

    for ans in answers {
        output::run(ans)?;
    }

    Ok(())
}
