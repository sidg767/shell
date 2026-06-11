use crate::error::shell_error::ShellError;

pub fn run() -> Result<i32, ShellError> {
    match std::env::current_dir() {
        Ok(dir) => {
            println!("{}", dir.display());
            Ok(0)
        }
        Err(e) => Err(ShellError::Io { source: e }),
    }
}
