use kinmu::generator_with_annealing::GeneratorWithAnnealing;
use kinmu::input_by_file::InputByFile;
use kinmu::lib::{Shift, StdFill, StdUpdate};

use kinmu::OutputTextOrHTML;

use getopts::Options;
use std::env;
use std::fs::OpenOptions;
use std::io;

/// 引数で指定しない場合に読み込むmain_configのパス
const DEFALUT_MAIN_CONFIG_PATH: &str = "example/simple_case/main_config.toml";
/// kinmuのバージョン
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// kinmuのリポジトリ
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
/// kinmuのライセンス
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
    opts.optflag("", "html", "htmlを出力");

    let mut out: Box<dyn io::Write> = Box::new(io::stdout());
    let mut use_color = true;

    // オプションの読み込み
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            kinmu::color::write(&mut out, "[エラー]", kinmu::color::Color::Red, use_color)?;
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
                kinmu::color::write(&mut out, "[エラー]", kinmu::color::Color::Red, use_color)?;
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
    let main_config_path = match load_option {
        Some(ref x) => x,
        None => DEFALUT_MAIN_CONFIG_PATH,
    };

    // html出力を選択
    let use_html = matches.opt_present("html");

    // 実行
    match kinmu::core::run(
        &mut InputByFile::new(main_config_path),
        &mut GeneratorWithAnnealing::new(StdFill, StdUpdate),
        {
            use Shift::*;
            &mut OutputTextOrHTML::new(
                use_html,
                &mut out,
                use_color,
                false,
                vec![H, O, I, N, K, Y],
                vec![N, I, A, K, O, H],
            )
        },
    ) {
        Ok(_) => {}
        Err(e) => {
            if use_html {
                writeln!(out, "<div style=\"white-space: pre-line;\">{:?}\n</div>", e)?
            } else {
                writeln!(out, "{:?}", e)?
            }
        }
    };

    Ok(())
}
