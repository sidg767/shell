use rustyline::hint::Hinter;
use rustyline::Context;

pub struct ShellHinter;

impl Hinter for ShellHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if line.is_empty() {
            return Some("Type a command...".to_string());
        }

        // Simple hints for common commands
        let trimmed = line.trim();
        if trimmed == "cd" {
            Some(" <directory>". Change directory.".to_string())
        } else if trimmed == "echo" {
            Some(" <text>. Print text to stdout.".to_string())
        } else if trimmed == "pwd" {
            Some(". Print working directory.".to_string())
        } else if trimmed == "exit" {
            Some(". Exit the shell.".to_string())
        } else if trimmed.starts_with("cd ") && trimmed.len() == 3 {
            Some("<directory>. Change to specified directory.".to_string())
        } else {
            None
        }
    }
}