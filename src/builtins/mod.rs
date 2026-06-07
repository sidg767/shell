pub mod cd;
pub mod echo;
pub mod pwd;
pub mod r#type;

use crate::error::shell_error::ShellError;

pub fn handle(cmd: &str, args: &[String]) -> Option<Result<i32, ShellError>> {
    match cmd {
        "exit" => {
            let code = args.first().and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);
            std::process::exit(code);
        }
        "cd" => Some(cd::run(args)),
        "echo" => {
            echo::run(args);
            Some(Ok(0))
        }
        "pwd" => {
            pwd::run();
            Some(Ok(0))
        }
        "type" => {
            r#type::run(args);
            Some(Ok(0))
        }
        _ => None,
    }
}
