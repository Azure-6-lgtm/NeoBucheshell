use crate::strap::repl;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use sysinfo::System;

/* =========================
Helper: expand ~
========================= */
fn expand_tilde(input: &str) -> PathBuf {
    if input == "~" || input.starts_with("~/") {
        if let Some(home) = env::var_os("HOME") {
            let mut path = PathBuf::from(home);

            if input.len() > 2 {
                path.push(&input[2..]); // skip "~/"
            }

            return path;
        }
    }

    PathBuf::from(input)
}

/* =========================
Basic commands
========================= */

pub fn aboutbsh() {
    let fun = "
            XX
        XXXX
     XXXX   
   XXX      
 XXX        
XX          
 XXX        
   XXX      
     XXXX   
        XXXX
           XX";
    println!("{}", fun);
    println!("NeoBucheshell is the remaster of Bucheshell");
    println!("It aims to be faster, well-optimized and more friendly");
}

pub fn bshversion() {
    println!("NeoBucheshell version 0.2.0 release");
}

pub fn help() {
    println!("Built in commands -> cd, about, ver, touch, mkdir, rmdir, rm, ls, export,pipe,channel,sysinfo");
    println!("You can set custom aliases in \"~/.neobucherc\"");
    println!("You can also set custom prompt in config file")
}

pub fn exit() {
    std::process::exit(0);
}

/* =========================
File system commands
========================= */

pub fn ls(args: &[&str]) {
    let path = args.get(0).copied().unwrap_or(".");
    let path = expand_tilde(path);

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                print!("{}  ", entry.file_name().to_string_lossy());
            }
            println!();
        }
        Err(e) => eprintln!("ls: {}", e),
    }
}

pub fn builtin_cd(args: &[&str]) {
    let target = if args.is_empty() {
        expand_tilde("~")
    } else {
        expand_tilde(args[0])
    };

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}", e);
    }
}

pub fn mkdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        let path = expand_tilde(dir);

        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("mkdir: {}", e);
        }
    } else {
        eprintln!("mkdir: missing operand");
    }
}

pub fn touch(args: &[&str]) {
    if let Some(file) = args.get(0) {
        let path = expand_tilde(file);

        if let Err(e) = File::create(path) {
            eprintln!("touch: {}", e);
        }
    } else {
        eprintln!("touch: missing file");
    }
}

pub fn rm(args: &[&str]) {
    if let Some(file) = args.get(0) {
        let path = expand_tilde(file);

        if let Err(e) = fs::remove_file(path) {
            eprintln!("rm: {}", e);
        }
    } else {
        eprintln!("rm: missing file");
    }
}

pub fn rmdir(args: &[&str]) {
    if let Some(dir) = args.get(0) {
        let path = expand_tilde(dir);

        if let Err(e) = fs::remove_dir(path) {
            eprintln!("rmdir: {}", e);
        }
    } else {
        eprintln!("rmdir: missing dir");
    }
}

pub fn cp(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("cp: src dst");
        return;
    }

    let src = expand_tilde(args[0]);
    let dst = expand_tilde(args[1]);

    if let Err(e) = fs::copy(src, dst) {
        eprintln!("cp: {}", e);
    }
}

pub fn mv(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("mv: src dst");
        return;
    }

    let src = expand_tilde(args[0]);
    let dst = expand_tilde(args[1]);

    if let Err(e) = fs::rename(src, dst) {
        eprintln!("mv: {}", e);
    }
}

pub fn channel(args: &[&str]) {
    //eprintln!("DEBUG channel args: {:?}", args);

    if args.is_empty() {
        eprintln!("USAGE: channel join(0)");
        return;
    }

    if args[0] == "join(0)" {
        //eprintln!("DEBUG: launching channel");
        repl::launch_channel();
    } else {
        eprintln!("Unknown command: {}", args[0]);
    }
}

pub fn which(args: &[&str]) {
    if args.is_empty() {
        eprintln!("Missing command");
        return;
    }
    let _path = match env::var("PATH") {
        _ => {
            todo!()
        }
    };
}

use std::process::{Command, Stdio};

pub fn pipe(args: &[&str]) {
    let pipe_pos = match args.iter().position(|&x| x == "|") {
        Some(pos) => pos,
        None => {
            eprintln!("pipe: missing '|'");
            return;
        }
    };

    let (left, right_with_pipe) = args.split_at(pipe_pos);
    let right = &right_with_pipe[1..];

    if left.is_empty() || right.is_empty() {
        eprintln!("pipe: invalid syntax");
        return;
    }

    let mut first = Command::new(left[0])
        .args(&left[1..])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout = first.stdout.take().unwrap();

    let mut second = Command::new(right[0])
        .args(&right[1..])
        .stdin(stdout)
        .spawn()
        .unwrap();

    second.wait().unwrap();
    first.wait().unwrap();
}

pub fn sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
}

pub fn ps() {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (pid, process) in sys.processes() {
        println!("[{pid}] {:?} {:?} \n", process.name(), process.disk_usage());
    }
}
