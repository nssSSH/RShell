use std::process::Command;
use std::{io, io::Write};

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something occured whilst reading your input.");
    input.trim().to_string()
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut parts = input();
        let mut parts = parts.split_whitespace();
        let command = parts.next().unwrap();

        let mut child = Command::new(command).args(parts).spawn().unwrap();

        child.wait();
    }
}
