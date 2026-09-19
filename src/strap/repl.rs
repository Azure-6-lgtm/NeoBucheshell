use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Variable {
    var_type: String,
    value: String,
}

pub fn launch_channel() {
    println!("Welcome to the NeoBucheshell REPL aka channel");
    println!("Version 0.1");
    //println!("Might contain bugs");

    let mut variables: HashMap<String, Variable> = HashMap::new();

    loop {
        print!("$CHANNEL>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command: Vec<&str> = input.split_whitespace().collect();

        if command.is_empty() {
            continue;
        }

        if command[0] == "exit" {
            break;
        } else {
            process_command(command, &mut variables);
        }
    }
}

fn process_command(command: Vec<&str>, variables: &mut HashMap<String, Variable>) {
    match command[0] {
        "let" => {
            if command.len() < 5 {
                println!("Usage: let <name> <type> = <value>");
                return;
            }

            let var_name = command[1];
            let var_type = command[2];
            let var_value = command[4];

            variables.insert(
                var_name.to_string(),
                Variable {
                    var_type: var_type.to_string(),
                    value: var_value.to_string(),
                },
            );

            println!("Created variable '{}'", var_name);
        }

        "echo" => {
            if command.len() < 2 {
                return;
            }

            println!("{}", command[1..].join(" "));
        }

        "varprint" => {
            if command.len() < 2 {
                println!("Usage: varprint <name>");
                return;
            }

            let var_name = command[1];

            match variables.get(var_name) {
                Some(variable) => {
                    println!("{}", variable.value);
                }

                None => {
                    println!("Variable '{}' not found", var_name);
                }
            }
        }

        "help" => {
            println!("Available Commands: ");
            println!("1.let");
            println!("2.varprint");
            println!("3.echo");
            println!("Type \" exit \" to exit ")
        }

        _ => {
            println!("Unknown command: {}", command[0]);
        }
    }
}
