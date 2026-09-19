use chrono::Local;
use dirs::home_dir;
use log::{error, info, warn};
use simplelog::*;
use std::fs::File;
use std::path::PathBuf;

pub fn init_logger() {
    // Force Rust to use local timezone (important for Termux/Android)
    // std::env::set_var("TZ", "Asia/Kolkata"); // replace with your timezone

    // ~/.neobucheshell.log
    let mut log_path = home_dir().expect("Could not find home directory");
    log_path.push(".neobucheshell.log");

    // Overwrite file every session
    let log_file = File::create(&log_path).expect("Could not create log file");

    // File-only logging, ignore rustyline debug
    let config = ConfigBuilder::new()
        .set_time_level(LevelFilter::Info) // include timestamp in log macros
        .build();

    WriteLogger::init(
        LevelFilter::Info, // log info+ (ignore debug)
        config,
        log_file,
    )
    .unwrap();
}

// Convenience logging functions with manual timestamp
pub fn log_info(msg: &str) {
    info!(
        "[{}][INFO] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}

pub fn log_warn(msg: &str) {
    warn!(
        "[{}][WARN] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}

pub fn log_error(msg: &str) {
    error!(
        "[{}][ERROR] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        msg
    );
}
