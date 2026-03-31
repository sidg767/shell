use std::env;

pub fn run(args: &[String]) {
    let home = if args.is_empty() || args[0] == "~" {
        env::var("HOME").unwrap_or("/".to_string())
    } else {
        args[0].clone()
    };

    if let Err(_) = env::set_current_dir(&home) {
        println!("cd: {}: No such file or directory", home);
    }
}