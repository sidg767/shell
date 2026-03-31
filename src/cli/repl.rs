use std::io::{self, Write};
use crate::lexer::tokenizer::form_tokens;
use crate::exec::executor::eval_command;
pub fn start() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens= {
            form_tokens(input.trim())
        };
        if tokens.is_empty() {
            continue;
        }
        let start = tokens.remove(0);   
        if start == "exit" {
            break;
        }
        eval_command(&start, tokens);
    }
}