use rustyline::{Editor, Result};
use crate::cli::completer::ShellCompleter;
use crate::cli::history;
use crate::lexer::tokenizer::form_tokens;
use crate::exec::executor::eval_command;

pub fn start() -> Result<()> {
    history::init_history().ok();
    
    let history_path = history::get_history_path_string();
    let mut rl = Editor::new()?;
    rl.set_helper(Some(ShellCompleter::new()));
    
    let _ = rl.load_history(&history_path);
    
    loop {
        let readline = rl.readline("$ ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let mut tokens: Vec<String> = form_tokens(&line.trim().to_string());
                if tokens.is_empty() {
                    continue;
                }
                let command = tokens.remove(0);
                if command == "exit" {
                    break;
                }
                eval_command(&command, tokens);
            }
            Err(_) => break,
        }
    }
    
    let _ = rl.save_history(&history_path);
    Ok(())
}
