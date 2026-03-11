use pathsearch::find_executable_in_path;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s.pop();
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
    if !comm_known.contains(&command) {
        println!("{}: command not found", &command);
        return;
    }
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
        } else if let Some(path) = find_executable_in_path(args[0]) {
            println!("{} is {}", args[0], path.display());
        } else {
            println!("{}: not found", args[0]);
        }
    }
}
