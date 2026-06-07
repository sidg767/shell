//! Current implementation has three layers: The Shell Runtime layer handles the core REPL loop,
//! prompt generation, command parsing, built-in commands, process execution, pipeline execution,
//! and history management. The Rustyline layer provides features like completion, syntax highlighting,
//! hints, validation, and history navigation. The Operating System layer is responsible for process
//! spawning, PATH lookup, current directory management, and pipes.
//!
//!
//!
//!

/// Represents errors that can occur while reading user input.
use rustyline::error::ReadlineError;
use rustyline::history::{DefaultHistory, History};
/// Provides: line editing, history, completion, keyboard key bindings, cursor movement,
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};

use super::completer::ShellCompleter;
use super::highlighter::ShellHighlighter;
use super::hinter::ShellHinter;
use super::validator::ShellValidator;

use std::path::Path;

const HISTORY_FILE: &str = ".shell_history";
const HISTORY_LIMIT: usize = 1000;

/// The Shell struct encapsulates the REPL loop and command handling logic. It uses Rustyline for
/// input handling, and delegates completion, highlighting, hints, and validation to helper structs.
/// The Shell supports built in commands like cd, echo, pwd, type, and exit, as well as external
///  commands found in the PATH. It also supports simple pipelines using the | operator. The REPL
///  loop handles user input, manages history, and gracefully handles interrupts and EOF. The Shell
///  is designed to be modular, with clear separation between the REPL logic, Rustyline integration,
///  and OS interactions.
pub struct Shell {
    editor: Editor<ShellHelper, DefaultHistory>,
}

pub struct ShellHelper {
    completer: ShellCompleter,
    highlighter: ShellHighlighter,
    hinter: ShellHinter,
    validator: ShellValidator,
}
impl rustyline::Helper for ShellHelper {}
impl rustyline::completion::Completer for ShellHelper {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<rustyline::completion::Pair>)> {
        self.completer.complete(line, pos, ctx)
    }
}
impl ShellHinter {
    pub fn new() -> Self {
        Self
    }
}
impl rustyline::hint::Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl rustyline::highlight::Highlighter for ShellHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: rustyline::highlight::CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}
impl rustyline::validate::Validator for ShellHelper {
    fn validate(
        &self,
        ctx: &mut rustyline::validate::ValidationContext,
    ) -> rustyline::Result<rustyline::validate::ValidationResult> {
        self.validator.validate(ctx)
    }
}
impl Shell {
    pub fn new() -> rustyline::Result<Self> {
        let helper = ShellHelper {
            completer: ShellCompleter::new(),
            highlighter: ShellHighlighter::new(),
            hinter: ShellHinter::new(),
            validator: ShellValidator::new(),
        };

        let mut editor = Editor::new()?;
        editor.set_helper(Some(helper));
        editor.history_mut().set_max_len(HISTORY_LIMIT);

        if Path::new(HISTORY_FILE).exists() {
            let _ = editor.load_history(HISTORY_FILE);
        }

        editor.bind_sequence(
            KeyEvent(KeyCode::Tab, Modifiers::NONE),
            EventHandler::Simple(Cmd::Complete),
        );

        Ok(Self { editor })
    }
    pub fn start(&mut self) -> rustyline::Result<()> {
        loop {
            let prompt = build_prompt();

            match self.editor.readline(&prompt) {
                Ok(line) => {
                    let line = line.trim().to_owned();

                    if line.is_empty() {
                        continue;
                    }

                    self.editor.add_history_entry(&line)?;
                    self.handle_line(&line);
                }

                Err(ReadlineError::Interrupted) => {
                    eprintln!("^C");
                    continue;
                }

                Err(ReadlineError::Eof) => {
                    break;
                }

                Err(err) => {
                    eprintln!("error: {}", err);
                    break;
                }
            }
        }

        let _ = self.editor.save_history(HISTORY_FILE);
        Ok(())
    }

    fn handle_line(&self, line: &str) {
        let tokens = crate::lexer::tokenizer::tokenize(line);
        let mut parser = crate::parser::parser::Parser::new(tokens);
        
        match parser.parse() {
            Ok(ast) => {
                if let Err(e) = crate::exec::executor::execute_ast(&ast) {
                    eprintln!("shell execution error: {}", e);
                }
            }
            Err(e) => eprintln!("shell parse error: {}", e),
        }
    }
}

fn build_prompt() -> String {
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    format!("{} $ ", cwd)
}



pub fn start() -> rustyline::Result<()> {
    Shell::new()?.start()
}
