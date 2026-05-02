use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline::completion::FilenameCompleter;

use crate::completer::ShellCompleter;
use crate::highlighter::ShellHighlighter;
use crate::hinter::ShellHinter;
use crate::validator::ShellValidator;

use std::process::{Command, Stdio};
use std::path::Path;

const HISTORY_FILE: &str = ".shell_history";
const HISTORY_LIMIT: usize = 1000;

#[derive(Debug)]
pub struct Shell {
    editor: Editor<ShellHelper, DefaultHistory>,
}

#[derive(rustyline::Helper)]
pub struct ShellHelper {
    completer:   ShellCompleter,
    highlighter: ShellHighlighter,
    hinter:      ShellHinter,
    validator:   ShellValidator,
}

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