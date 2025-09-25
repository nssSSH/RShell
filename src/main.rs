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

        let command = input();
        let mut child = Command::new(command).spawn().unwrap();

        child.wait();
    }
}
