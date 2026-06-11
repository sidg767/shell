use std::process::{Command as ProcessCommand, Stdio};
use crate::builtins;
use crate::exec::redirect::handle_redirects;
use crate::utils::path::find_executable;
use crate::parser::ast::{AstNode, AndOrList, Pipeline, Command as AstCommand, LogicOp};
use crate::error::shell_error::ShellError;

pub fn execute_ast(node: &AstNode) -> Result<i32, ShellError> {
    execute_and_or_list(&node.list)
}

fn execute_and_or_list(list: &AndOrList) -> Result<i32, ShellError> {
    let mut last_status = 0;
    
    // An empty list means nothing to do
    if list.pipelines.is_empty() {
        return Ok(0);
    }

    let mut i = 0;
    while i < list.pipelines.len() {
        let (pipeline, logic_op) = &list.pipelines[i];
        
        last_status = execute_pipeline(pipeline)?;

        match logic_op {
            Some(LogicOp::And) => {
                if last_status != 0 {
                    // Skip next pipeline if && fails
                    i += 1;
                }
            }
            Some(LogicOp::Or) => {
                if last_status == 0 {
                    // Skip next pipeline if || succeeds
                    i += 1;
                }
            }
            Some(LogicOp::Semi) | None => {}
        }
        
        i += 1;
    }

    Ok(last_status)
}

fn execute_pipeline(pipeline: &Pipeline) -> Result<i32, ShellError> {
    if pipeline.commands.is_empty() {
        return Ok(0);
    }

    if pipeline.commands.len() == 1 {
        return execute_command(&pipeline.commands[0], None, None);
    }

    let mut last_status = 0;
    let mut previous_stdout: Option<std::process::ChildStdout> = None;
    let len = pipeline.commands.len();

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        let stdin = previous_stdout.take().map(Stdio::from);

        let stdout = if i < len - 1 {
            Some(Stdio::piped())
        } else {
            None
        };
        
        let mut builder = ProcessCommand::new(&cmd.name);
        builder.args(&cmd.args);
        
        let redirects = handle_redirects(&cmd.redirects)?;
        
        if let Some(s) = redirects.stdin {
            builder.stdin(s);
        } else if let Some(s) = stdin {
            builder.stdin(s);
        }

        if let Some(s) = redirects.stdout {
            builder.stdout(s);
        } else if let Some(s) = stdout {
            builder.stdout(s);
        }

        match builder.spawn() {
            Ok(mut child) => {
                if i < len - 1 {
                    previous_stdout = child.stdout.take();
                } else {
                    let status = child.wait().map_err(|e| ShellError::Io { source: e })?;
                    last_status = status.code().unwrap_or(0);
                }
            }
            Err(e) => {
                eprintln!("{}: {}", cmd.name, e);
                last_status = 127;
                break;
            }
        }
    }

    Ok(last_status)
}

fn execute_command(cmd: &AstCommand, stdin: Option<Stdio>, stdout: Option<Stdio>) -> Result<i32, ShellError> {
    let redirects = handle_redirects(&cmd.redirects)?;

    // builtins
    if let Some(result) = builtins::handle(&cmd.name, &cmd.args) {
        return result;
    }

    // external command
    if let Some(_path) = find_executable(&cmd.name) {
        let mut builder = ProcessCommand::new(&cmd.name);
        builder.args(&cmd.args);

        if let Some(s) = redirects.stdin {
            builder.stdin(s);
        } else if let Some(s) = stdin {
            builder.stdin(s);
        }

        if let Some(s) = redirects.stdout {
            builder.stdout(s);
        } else if let Some(s) = stdout {
            builder.stdout(s);
        }

        match builder.spawn() {
            Ok(mut child) => {
                let status = child.wait().map_err(|e| ShellError::Io { source: e })?;
                Ok(status.code().unwrap_or(0))
            }
            Err(e) => {
                eprintln!("{}: {}", cmd.name, e);
                Ok(127)
            }
        }
    } else {
        eprintln!("{}: command not found", cmd.name);
        Ok(127)
    }
}
