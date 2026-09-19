use std::fs::{self, OpenOptions};
use std::io::Write;
use std::env;

fn get_alias_file() -> String {
    let home = env::var("HOME").unwrap();
    format!("{}/.neobucherc", home)
}

pub fn resolve(input: &str) -> String {
    let path = get_alias_file();
    let content = fs::read_to_string(path).unwrap_or_default();

    let mut parts = input.split_whitespace();
    let command = match parts.next() {
        Some(cmd) => cmd,
        None => return input.to_string(),
    };

    let args: Vec<&str> = parts.collect();

    for line in content.lines() {
        let line = line.trim();

        if !line.starts_with("alias ") {
            continue;
        }

        let rest = &line[6..];

        if let Some((left, right)) = rest.split_once('=') {
            let name = left.trim();
            let value = right.trim();

            if command == name {
                if args.is_empty() {
                    return value.to_string();
                } else {
                    return format!("{} {}", value, args.join(" "));
                }
            }
        }
    }

    input.to_string()
}

pub fn add_alias(input: &str) {
    let path = get_alias_file();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    writeln!(file, "{}", input).unwrap();
    println!("Alias added");
}
