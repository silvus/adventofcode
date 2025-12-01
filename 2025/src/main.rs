mod day_1;

use clap::Parser;
use log::{Level, LevelFilter, Record};
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[arg(help = "Day number to run")]
    day: usize,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let debug_mode = args.debug;

    env_logger::Builder::from_default_env()
        .filter_level(if debug_mode {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .format(|buf, record: &Record| {
            // Pad raw level to 5 chars
            let level_raw = format!("{:<5}", record.level());
            let level_colored = match record.level() {
                Level::Error => format!("\x1b[31m{}\x1b[0m", level_raw),
                Level::Warn => format!("\x1b[33m{}\x1b[0m", level_raw),
                Level::Info => format!("\x1b[32m{}\x1b[0m", level_raw),
                Level::Debug => format!("\x1b[34m{}\x1b[0m", level_raw),
                Level::Trace => format!("\x1b[36m{}\x1b[0m", level_raw),
            };

            // let ts = buf.timestamp_millis();
            // writeln!(buf, "{} {} {}", ts, level, record.args())
            writeln!(buf, "[{}] {}", level_colored, record.args())
        })
        .init();

    match day {
        1 => day_1::solve(),
        // 2 => day_2::solve(),
        _ => log::error!("Unsupported day"),
    }
}
