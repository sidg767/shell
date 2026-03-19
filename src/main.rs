#[allow(unused_imports)]
use pathsearch::find_executable_in_path;
use std::io::{self, Write};
use std::{self, 
    env, 
    str, 
    error::Error, 
    fs::DirEntry,
    collections::HashMap,
    ops::ControlFlow,
    io::Write};

 enum State{
    Normal,
    SingleQuote,
    DoubleQuote,
    Escape,
 }   
 fn FormTokens(input: &str)->Vec<String>{
   let mut tokens=Vec::new();
   let mut curr_token=String::new();
   for c in input.chars(){
    match state{
        State::Normal=> match c{

        }
        State::SingleQuote=>{

        }
        State::DoubleQuote=>{   

        }
        State::Escape=>{

        }
        
    }
   }
   if !curr_token.is_empty(){
            tokens.push(current);
        }
        tokens
 }
fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let v: Vec<&str> = s.trim().split_whitespace().collect();
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
    let comm_known = ["exit", "echo", "type", "pwd", "cd"];
    if command == "pwd" {
        println!("{}", std::env::current_dir().unwrap().display());
        return;
    }
    if command == "echo" {
        if args[0] == "'" {}
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

    if command == "cd" {
        if args.is_empty() {
            let home = env::var("HOME").unwrap_or("/".to_string());
            if let Err(e) = env::set_current_dir(home) {
                println!("cd: {}: No such file or directory", e);
            }
        } 
        else if args[0] =="~" {
            let home = env::var("HOME").unwrap_or("/".to_string());
            if let Err(e)=env::set_current_dir(&home){
                println!("cd: {}: No such file or directory", e);
            }
        }       
        else {
            if let Err(_) = env::set_current_dir(PathBuf::from(args[0])) {
                println!("cd: {}: No such file or directory", &args[0]);
            }
        }
        return;
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
