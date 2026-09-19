//import necessary crates
use crate::logger::buchelog::{log_error, log_info, log_warn};
use crate::utils::alias;
use crate::utils::run;
use colored::*;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Command, Stdio};

//constants
const USERPATH: &str = "/usr/bin/";
const TERMUXPATH: &str = "/data/data/com.termux/files/usr/bin/";
const SHELLPATH: &str = "/bin/";

pub fn checkcmd(input_command: &str) {
    if input_command.is_empty() {
        log_warn("Input command empty. Returning");
        return;
    }

    // Step 1: Resolve alias FIRST
    let resolved_input = alias::resolve(input_command);

    // Step 2: Split resolved input
    let mut parts: Vec<&str> = resolved_input.split_whitespace().collect();
    let mut output_file: Option<&str> = None;

    // Step 3: Handle output redirection
    if let Some(pos) = parts.iter().position(|&x| x == ">>") {
        if pos + 1 < parts.len() {
            output_file = Some(parts[pos + 1]);
            parts.truncate(pos);
        } else {
            println!("{}", "Syntax error: no file specified for >>".red());
            return;
        }
    }

    // Step 4: Safe command extraction
    let cmd = match parts.get(0) {
        Some(c) => *c,
        None => return,
    };

    let cmdargs: Vec<&str> = parts[1..].to_vec();

    // -----------------
    // Built-in commands
    // -----------------

    if cmd == ".." && cmdargs.is_empty() {
        std::env::set_current_dir("..").unwrap();
        return;
    }

    // export
    if cmd == "export" && cmdargs.len() >= 3 && cmdargs[1] == "=" {
        unsafe {
            std::env::set_var(cmdargs[0], cmdargs[2]);
            log_info("Set environment variable");
        }
        return;
    }

    // echo (basic)
    if cmd == "echo" && cmdargs.len() == 1 {
        let arg = cmdargs[0];

        if arg.starts_with('$') {
            let key = &arg[1..];

            if let Ok(val) = std::env::var(key) {
                println!("{}", val);
            }
        } else {
            println!("{}", arg);
        }
        return;
    }

    // -----------------
    // Path resolution
    // -----------------

    let termuxpath = format!("{}{}", TERMUXPATH, cmd);
    let userpath = format!("{}{}", USERPATH, cmd);
    let shellpath = format!("{}{}", SHELLPATH, cmd);

    let mut command_to_run = if Path::new(&termuxpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command successfully {}", termuxpath));
        Command::new(&termuxpath)
    } else if Path::new(&userpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command successfully {}", userpath));
        Command::new(&userpath)
    } else if Path::new(&shellpath).exists() && !checkforbuiltin(cmd) {
        log_info(&format!("Ran command successfully {}", shellpath));
        Command::new(&shellpath)
    } else {
        checkutils(cmd, &cmdargs);
        log_warn("Passing unknown command to utils checker");
        return;
    };

    // -----------------
    // Apply arguments
    // -----------------
    command_to_run.args(&cmdargs);

    // -----------------
    // Output redirection
    // -----------------
    if let Some(file) = output_file {
        let file_handle = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file)
            .expect("Failed to open file");

        command_to_run.stdout(Stdio::from(file_handle));
    }

    // -----------------
    // Execute
    // -----------------
    command_to_run.status().expect("Bucheshell failed to run");
}

// -----------------
// Built-in handler
// -----------------

pub fn checkutils(utilcmd: &str, cmdargs: &Vec<&str>) {
    match utilcmd {
        "cd" => {
            log_info("Ran builtin command: cd");
            run::builtin_cd(cmdargs);
        }
        "ver" => {
            log_info("Ran builtin command: ver");
            run::bshversion();
        }
        "help" => {
            log_info("Ran builtin command: help");
            run::help();
        }
        "about" => {
            log_info("Ran builtin command: about");
            run::aboutbsh();
        }
        "exit" => {
            log_info("Exiting shell");
            run::exit();
        }
        "mkdir" => {
            log_info("Ran builtin command: mkdir");
            run::mkdir(cmdargs);
        }
        "rm" => {
            log_info("Ran builtin command: rm");
            run::rm(cmdargs);
        }
        "rmdir" => {
            log_info("Ran builtin command: rmdir");
            run::rmdir(cmdargs);
        }
        "touch" => {
            log_info("Ran builtin command: touch");
            run::touch(cmdargs);
        }
        "cp" => {
            log_info("Ran builtin command: cp");
            run::cp(cmdargs);
        }
        "ls" => {
            log_info("Ran builtin command: ls");
            run::ls(cmdargs);
        }
        "mv" => {
            log_info("Ran builtin command: mv");
            run::mv(cmdargs);
        }
        "pipe" => {
            log_info("Ran builtin command: pipe");
            run::pipe(cmdargs);
        }
        "sysinfo" => {
            log_info("Ran builtin command: sysinfo");
            run::sysinfo();
        }
        "ps" => {
            log_info("Ran builtin command: ps");
            run::ps();
        }
        "channel" => {
            log_info("Ran builtin command: channel");
            run::channel(cmdargs);
        }
        "which" => {
            log_info("Ran builtin command: which");
            run::which(cmdargs);
        }
        _ => {
            log_error("Unknown command");
            println!("{}", "Command is not available!".yellow());
        }
    };
}

// -----------------
// Builtin checker
// -----------------

fn checkforbuiltin(cmd: &str) -> bool {
    matches!(
        cmd,
        "mkdir"
            | "rm"
            | "touch"
            | "mv"
            | "cp"
            | "rmdir"
            | "ls"
            | "pipe"
            | "sysinfo"
            | "ps"
            | "channel"
    )
}

// -----------------
// Args deciphering
// -----------------

pub fn decipher_args(args: Vec<String>) {
    if args.is_empty() {
        //do nothing
        return;
    }

    match args[1].as_str() {
        "-rc" => {
            if args.len() >= 3 {
                // buchesh -rc apt install bucheshell
                let cmdtorun = args[2].clone();
                let cmdtorunargs: Vec<String> = {
                    if args.len() >= 4 {
                        args[3..].to_vec()
                    } else {
                        vec![]
                    }
                };
                Command::new(cmdtorun)
                    .args(cmdtorunargs)
                    .status()
                    .expect("Failed to run command");
                std::process::exit(0);
            }
        }
        "-rf" => {
            println!("Under Construction :)");
            std::process::exit(0);
        }
        "-help" => {
            println!(
                "Built in commands -> cd, about, ver, touch, mkdir, rmdir, rm, ls, export, pipe, sysinfo, ps, channel."
            );
            println!("You can set custom aliases in \"~/.neobucherc\"");
            println!("You can also set custom prompt in the config file");
            println!("You can enter programming enviorment by typing \"channel start \" ");
            println!("Visit NeoBucheshell github page to learn more");
            std::process::exit(0);
        }
        "-ver" => {
            println!("NeoBucheshell Version 0.1.0");
            std::process::exit(0)
        }
        _ => {}
    }
}
