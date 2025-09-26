use colored::Colorize;
use std::process::Command;
use std::{io, io::Write};

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
/*
fn shell_cd(path: Option<&str>) {
    
}
*/

fn main() {
    loop {
        print_colored("$ ", "red");
        io::stdout().flush().unwrap();

        let input_line = input();
        let mut parts = input_line.split_whitespace();
        let command = parts.next().unwrap();
        let mut child = Command::new(command).args(parts).spawn().unwrap();

        child.wait();
    }
}
