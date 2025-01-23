use ::kinmu::generator;
use ::kinmu::io::input;
use ::kinmu::io::output;
use ::kinmu::utils::color;

use getopts::Options;
use std::env;
use std::fs::OpenOptions;
use std::io;

const DEFALUT_MAIN_CONFIG_PATH: &str = "example/main_config.toml";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
const LICENSE: &str = env!("CARGO_PKG_LICENSE");

fn write_usage<W: io::Write>(out: &mut W, program: &str, opts: Options) -> io::Result<()> {
    let brief = format!("Usage: {} PATH [options]", program);
    write!(out, "{}", opts.usage(&brief))
}

fn main() -> io::Result<()> {
    // オプションを登録
    let mut opts = Options::new();
    opts.optopt(
        "o",
        "output",
        "select output path and run 出力ファイルを指定して実行",
        "PATH",
    );
    opts.optopt(
        "l",
        "load",
        "select config path and run 設定ファイルを指定して実行",
        "PATH",
    );
    opts.optflag("h", "help", "show help ヘルプを表示");
    opts.optflag("v", "version", "show version バージョンを表示");

    let mut out: Box<dyn io::Write> = Box::new(io::stdout());
    let mut use_color = true;

    // オプションの読み込み
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            color::write(&mut out, "[エラー]", color::Color::Red, use_color)?;
            writeln!(out, " オプションが不正です")?;
            writeln!(out, "理由: {}", f)?;
            write_usage(&mut out, &program, opts)?;
            return Ok(());
        }
    };

    // 出力先を選択
    let mut out: Box<dyn io::Write> = match matches.opt_str("o") {
        Some(x) => match OpenOptions::new().create(true).append(true).open(x) {
            Ok(f) => {
                use_color = false;
                Box::new(f)
            }
            Err(e) => {
                color::write(&mut out, "[エラー]", color::Color::Red, use_color)?;
                writeln!(out, " ファイルの読み込みに失敗しました")?;
                writeln!(out, "理由: {}", e)?;
                return Ok(());
            }
        },
        None => Box::new(io::stdout().lock()),
    };

    // ヘルプを表示
    if matches.opt_present("h") {
        write_usage(&mut out, &program, opts)?;
        return Ok(());
    };

    // バージョンを表示
    if matches.opt_present("v") {
        writeln!(out, "kinmu {}", VERSION)?;
        writeln!(out, "License: {}", LICENSE)?;
        writeln!(out, "Repository: {}", REPOSITORY)?;
        return Ok(());
    };

    // loadオプションの読み込み
    let load_option = matches.opt_str("l");
    let main_file_path = match load_option {
        Some(ref x) => x,
        None => DEFALUT_MAIN_CONFIG_PATH,
    };

    // 実行
    match run(&mut out, main_file_path, use_color) {
        Ok(_) => {}
        Err(e) => writeln!(out, "{:?}", e)?,
    };

    Ok(())
}

fn run<W: io::Write>(mut out: &mut W, main_file_path: &str, use_color: bool) -> anyhow::Result<()> {
    let config = input::load_config(main_file_path)?;

    let answers = generator::run(&config)?;

    for ans in answers {
        output::run(&mut out, ans, use_color)?;
    }

    Ok(())
}
