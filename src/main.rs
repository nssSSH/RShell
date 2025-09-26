use colored::Colorize;
use std::process::Command;
use std::{io, io::Write};
use std::env;
use std::path::{Path, PathBuf};

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something occured whilst reading your input.");
    input.trim().to_string()
}

fn print_colored(text: &str, colour: &str) {
    match colour {
        "red" => print!("{}", text.red()),
        "green" => print!("{}", text.green()),
        "yellow" => print!("{}", text.yellow()),
        "cyan" => print!("{}", text.cyan()),
        "purple" => print!("{}", text.purple()),
        _ => print!("{} Color not supported!", text.red()),
    };
}

fn shell_pwd()  -> io::Result<PathBuf>{
     env::current_dir()   
}

fn shell_cd_minus() {
    match std::env::var("OLDPWD") {
        Ok(old_pwd) => std::env::set_current_dir(&old_pwd).unwrap(),
        Err(e) => eprintln!("{e}")
    }    
}

fn shell_cd_home() {
    match std::env::var("HOME") {
        Ok(home) => std::env::set_current_dir(&home).unwrap(),
        Err(e) => eprintln!("{e}")
    }
}

fn shell_cd<P: AsRef<Path>>(path: P) {
    if let Err(e) = std::env::set_current_dir(path) {
        eprintln!("{e}");
    }
}

const BUILT_IN_COMMANDS: [&str; 5] = ["cd", "cd -", "cd <path>", "pwd", "exit"];

fn main() {
    ctrlc::set_handler(move || {
        print!("");
    }).expect("Something went wrong while setting CTRL+C handler.");
    loop {
        print_colored("$ ", "red");
        io::stdout().flush().unwrap();

        let input_line = input();
        let mut parts = input_line.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue
        };
        let args: Vec<&str> = parts.clone().collect();
        if command == "pwd" {
            match shell_pwd(){
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("{}", e)            }
        } else if command == "cd"{
            if args.len() == 0 {
                shell_cd_home();
            } else if args[0] == "-" {
                shell_cd_minus();
            } else if args[0] != "-"{
                shell_cd(args[0]);
                
            } else {
                println!("cd..");
            }
                
        } else if command == "exit"  {
            break
        } else if command == "help" {
            println!("Built in commands: {:?}", BUILT_IN_COMMANDS);
        } else {
            let mut child = Command::new(command)
                .args(parts)
                .spawn()
                .unwrap();

            child.wait();
        }
    }
}
