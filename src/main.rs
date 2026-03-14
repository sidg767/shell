#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::process::Command;
use std::str;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let v: Vec<&str> = s.split_whitespace().collect();
        if v.is_empty() {
            continue;
        }
        let start = v[0];
        if start == "exit" {
            break;
        }
        eval_command(start, v[1..v.len()].to_vec());
    }
}
fn eval_command(command: &str, args: Vec<&str>) {
    let comm_known = ["exit", "echo", "type"];
    if command == "echo" {
        for arg in &args {
            print!("{} ", arg);
        }
        println!();
        return;
    }

    if command == "type" {
        if comm_known.contains(&args[0]) {
            println!("{} is a shell builtin", &args[0]);
            return;
        } else if let Some(path) = find_executable_in_path(args[0]) {
            println!("{} is {}", args[0], path.display());
            return;
        } else {
            println!("{}: not found", args[0]);
            return;
        }
    }
    if let Some(_path) = find_executable_in_path(command) {
        let output = Command::new(command)
            .args(&args)
            .output()
            .expect("Invalid command");
        if output.status.success() {
            let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF-8 output");
            print!("{}", stdout);
        } else {
            println!("{}: command not found", &command);
        }
    } else {
        println!("{}: command not found", &command);
    }
}
