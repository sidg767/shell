use std::fs::File;
use std::io::Write;
use std::process::Output;

pub fn handle_redirects(mut args: Vec<String>) 
    -> (Vec<String>, Option<String>, Option<String>) 
{
    let mut stdout_redirect = None;
    let mut stderr_redirect = None;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            ">" => {
                stdout_redirect = Some(args[i + 1].clone());
                args.drain(i..=i + 1);
            }
            "2>" => {
                stderr_redirect = Some(args[i + 1].clone());
                args.drain(i..=i + 1);
            }
            _ => i += 1,
        }
    }

    (args, stdout_redirect, stderr_redirect)
}

pub fn write_output(
    output: Output,
    stdout_redirect: Option<String>,
    stderr_redirect: Option<String>,
) {
    if let Some(file) = stdout_redirect {
        let mut f = File::create(file).unwrap();
        f.write_all(&output.stdout).unwrap();
    } else {
        print!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if let Some(file) = stderr_redirect {
        let mut f = File::create(file).unwrap();
        f.write_all(&output.stderr).unwrap();
    } else {
        eprint!("{}", String::from_utf8_lossy(&output.stderr));
    }
}