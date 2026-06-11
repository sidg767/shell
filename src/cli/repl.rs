use rustyline::{Editor, Result};
use crate::cli::completer::ShellCompleter;
use crate::cli::history;
use crate::lexer::tokenizer::tokenize;
use crate::parser::parser::Parser;
use crate::exec::executor::execute_ast;

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
                let _ = rl.add_history_entry(&line);
                let tokens = tokenize(line.trim());
                if tokens.is_empty() {
                    continue;
                }
                
                if let Some(crate::lexer::tokens::Token::Word(w)) = tokens.first()
                    && w == "exit" {
                        break;
                    }
                
                let mut parser = Parser::new(tokens);
                match parser.parse() {
                    Ok(ast) => {
                        if let Err(e) = execute_ast(&ast) {
                            eprintln!("Error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Parse error: {}", e);
                    }
                }
            }
            Err(_) => break,
        }
    }
    
    let _ = rl.save_history(&history_path);
    Ok(())
}
