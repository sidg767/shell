use rustyline::highlight::Highlighter;
use std::borrow::Cow;

const RESET:   &str = "\x1b[0m";
const MAGENTA: &str = "\x1b[35m";
const GREEN:   &str = "\x1b[32m";
const YELLOW:  &str = "\x1b[33m";
const BOLD:    &str = "\x1b[1m";

#[derive(Default)]
pub struct ShellHighlighter;
