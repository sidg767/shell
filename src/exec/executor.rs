use std::process::Command;

use crate::builtins;
use crate::exec::redirect::handle_redirects;
use crate::utils::path::find_executable;

pub fn eval_command(command: &str, args: Vec<String>) {
    // handle redirects first
    let (args, stdout_redirect, stderr_redirect) = handle_redirects(args);

    // builtins
    if builtins::handle(command, &args) {
        return;
    }

    // external command
    if let Some(_path) = find_executable(command) {
        let output = Command::new(command)
            .args(&args)
            .output()
            .expect("failed to execute");

        crate::exec::redirect::write_output(output, stdout_redirect, stderr_redirect);
    } else {
        println!("{}: command not found", command);
    }
}