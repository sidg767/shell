use std::env;
use crate::error::shell_error::ShellError;

pub fn run(args: &[String]) -> Result<i32, ShellError> {
    let home = if args.is_empty() || args[0] == "~" {
        env::var("HOME").unwrap_or("/".to_string())
    } else {
        args[0].clone()
    };

    if env::set_current_dir(&home).is_err() {
        eprintln!("cd: {}: No such file or directory", home);
        return Ok(1); // Usually cd failing sets exit code 1
    }
    
    Ok(0)
}
