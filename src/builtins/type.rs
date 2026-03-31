use crate::utils::path::find_executable;

pub fn run(args: &[String]) {
    let builtins = ["cd", "echo", "pwd", "type"];

    if args.is_empty() {
        println!("type: missing argument");
        return;
    }

    if builtins.contains(&args[0].as_str()) {
        println!("{} is a shell builtin", args[0]);
    } else if let Some(path) = find_executable(&args[0]) {
        println!("{} is {}", args[0], path.display());
    } else {
        println!("{}: not found", args[0]);
    }
}