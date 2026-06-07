use std::fs::{File, OpenOptions};
use std::process::Stdio;

use crate::error::shell_error::ShellError;
use crate::parser::ast::Redirect;

pub struct StdioRedirects {
    pub stdin: Option<Stdio>,
    pub stdout: Option<Stdio>,
}

pub fn handle_redirects(redirects: &[Redirect]) -> Result<StdioRedirects, ShellError> {
    let mut stdin_res = None;
    let mut stdout_res = None;

    for redirect in redirects {
        match redirect {
            Redirect::In(file) => {
                let f = File::open(file).map_err(|e| ShellError::OpenFile {
                    path: file.clone(),
                    source: e,
                })?;
                stdin_res = Some(Stdio::from(f));
            }
            Redirect::Out(file) => {
                let f = File::create(file).map_err(|e| ShellError::OpenFile {
                    path: file.clone(),
                    source: e,
                })?;
                stdout_res = Some(Stdio::from(f));
            }
            Redirect::Append(file) => {
                let f = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file)
                    .map_err(|e| ShellError::OpenFile {
                        path: file.clone(),
                        source: e,
                    })?;
                stdout_res = Some(Stdio::from(f));
            }
        }
    }

    Ok(StdioRedirects {
        stdin: stdin_res,
        stdout: stdout_res,
    })
}
