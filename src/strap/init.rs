/*

This is the location where the shell actually starts
Be careful while editing this

*/
//Import necessary crates and utils
use crate::handler::check;
use crate::logger::buchelog::log_info;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::env;
use std::fs;
use std::io::{self, Write};

fn get_display_dir() -> String { // Gets your current directory and returns it in the form of string
    let dir = env::current_dir().unwrap();
    let home = env::var("HOME").unwrap(); 

    let path = dir.display().to_string();

    if path.starts_with(&home) { // replaces the systems homen with "~" . Necessary on android platforms
        path.replacen(&home, "~", 1)
    } else {
        path
    }
}

fn get_config_file() -> String { //Although we can harcode the config file to "~/.neobucherc" . This is necessary as std::fs does not fucking know what is "~"
    let home = env::var("HOME").unwrap();
    format!("{}/.neobucherc", home) //This used to be .bucherc
}

fn get_prompt() -> String {
    // This funcion is the actual fun thing
    // It reads the config file
    // And finds the prompt syntax else fallsback
    let content = fs::read_to_string(get_config_file()).unwrap_or_default();
    for line in content.lines() {
        let line = line.trim();

        if let Some(rest) = line.strip_prefix("prompt =") {
            return rest.trim().to_string();
        }
    }

    String::from("$USER@$HOST : $PWD$") // fallback in case no custom prompt is specified
}

fn render_prompt(prompt: &str) -> String {
    // Instead of parsing the prompt syntax
    // We just replace the Prompt variables with then data that we collect
    prompt
        .replace("$USER", &get_user())
        .replace("$HOST", &get_host())
        .replace("$PWD", &get_display_dir())
        .replace("$TIME", &get_time())
}

fn get_user() -> String { // It gets your username by checking the enviorment variable
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

fn get_host() -> String {
    /*
        This function gets your hostname by checking /proc/sys/kernel/hostname
    */
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "localhost".to_string())
        .trim()
        .to_string()
}

use chrono::Local; // Chrono here because i felt like it

fn get_time() -> String {
    Local::now().format("%H:%M").to_string()
}

pub fn init() {
    /*
        This prints the prompt and actually takes input and creates command histroy
    */
    let prompt_template = get_prompt();
    let mut rl = DefaultEditor::new().unwrap();
    log_info("Bootstrap started");
    loop {
        let _cpath = get_display_dir();
        let prompt = render_prompt(&prompt_template);
        let input_command = match rl.readline(&prompt) {
            Ok(input) => {
                rl.add_history_entry(input.as_str());
                input
            }
            Err(ReadlineError::Interrupted) => {
                println!(); // Ctrl+C does not exit shell
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
        check::checkcmd(&input_command); // passed to handler
        log_info("Command passed to handler"); 
    }
}
