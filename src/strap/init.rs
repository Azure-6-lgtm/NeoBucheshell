use crate::handler::check;
use crate::logger::buchelog::{log_error, log_info, log_warn};
use colored::*;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::fs;
use std::io::{self, Write};

fn get_display_dir() -> String {
    let dir = env::current_dir().unwrap();
    let home = env::var("HOME").unwrap();

    let path = dir.display().to_string();

    if path.starts_with(&home) {
        path.replacen(&home, "~", 1)
    } else {
        path
    }
}

fn get_config_file() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.neobucherc", home)
}

fn get_prompt() -> String {
    let content = fs::read_to_string(get_config_file()).unwrap_or_default();
    for line in content.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("prompt =") {
            return rest.trim().to_string();
        }
    }

    String::from("$USER@$HOST : $PWD$")
}

fn render_prompt(prompt: &str) -> String {
    prompt
        .replace("$USER", &get_user())
        .replace("$HOST", &get_host())
        .replace("$PWD", &get_display_dir())
        .replace("$TIME", &get_time())
}

fn get_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

fn get_host() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "localhost".to_string())
        .trim()
        .to_string()
}

use chrono::Local;

fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}

pub fn init() {
    let prompt_template = get_prompt();
    let mut rl = DefaultEditor::new().unwrap();
    log_info("Bootstrap started");
    loop {
        let cpath = get_display_dir();
        let prompt = render_prompt(&prompt_template);
        let input_command = match rl.readline(&prompt) {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                input
            }
            Err(ReadlineError::Interrupted) => {
                println!(); // Ctrl+C
                continue;
            }
            Err(ReadlineError::Eof) => {
                break; // Ctrl+D exits shell
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        };
        let input_command = input_command.trim();
        io::stdout().flush().unwrap();
        check::checkcmd(&input_command);
        log_info("Command passed to handler");
    }
}
