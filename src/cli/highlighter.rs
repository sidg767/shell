use rustyline::highlight::Highlighter;
use std::borrow::Cow;

const RESET:   &str = "\x1b[0m";
const MAGENTA: &str = "\x1b[35m";
const GREEN:   &str = "\x1b[32m";
const YELLOW:  &str = "\x1b[33m";
const BOLD:    &str = "\x1b[1m";

#[derive(Default)]
pub struct ShellHighlighter;
impl ShellHighlighter {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Highlighter for ShellHighlighter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        if !needs_highlighting(line) {
            return Cow::Borrowed(line);
        }

        let mut out = String::with_capacity(line.len() * 4);
        let mut chars = line.char_indices().peekable();
        let mut state = HighlightState::Normal;

        while let Some((byte_pos, ch)) = chars.next() {
            match (&state, ch) {
                (HighlightState::Escaped, _) => {
                    push_colored(&mut out, &line[byte_pos..byte_pos + ch.len_utf8()], MAGENTA);
                    state = HighlightState::Normal;
                }

                (HighlightState::Normal, '\\') => {
                    out.push_str(MAGENTA);
                    out.push(ch);
                    state = HighlightState::Escaped;
                }

                (HighlightState::Normal, '\'') => {
                    out.push_str(GREEN);
                    out.push(ch);
                    state = HighlightState::SingleQuote;
                }

                (HighlightState::Normal, '"') => {
                    out.push_str(GREEN);
                    out.push(ch);
                    state = HighlightState::DoubleQuote;
                }

                (HighlightState::SingleQuote, '\'') => {
                    out.push(ch);
                    out.push_str(RESET);
                    state = HighlightState::Normal;
                }

                (HighlightState::DoubleQuote, '"') => {
                    out.push(ch);
                    out.push_str(RESET);
                    state = HighlightState::Normal;
                }

                (HighlightState::SingleQuote, _)
                | (HighlightState::DoubleQuote, _) => {
                    out.push_str(YELLOW);
                    out.push(ch);
                    out.push_str(RESET);
                }

                (HighlightState::Normal, '|' | ';' | '&') => {
                    out.push_str(BOLD);
                    out.push(ch);
                    out.push_str(RESET);
                }

                (HighlightState::Normal, _) => {
                    out.push(ch);
                }
            }
        }

        if state != HighlightState::Normal {
            out.push_str(RESET);
        }

        Cow::Owned(out)
    }

    fn highlight_char(&self, line: &str, pos: usize, _kind: rustyline::highlight::CmdKind) -> bool {
        line.as_bytes().get(pos).map_or(false, |&b| {
            matches!(b, b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'"' | b'\'')
        })
    }
}

#[derive(Debug, PartialEq)]
enum HighlightState {
    Normal,
    Escaped,
    SingleQuote,
    DoubleQuote,
}

fn needs_highlighting(line: &str) -> bool {
    line.bytes().any(|b| matches!(b, b'\\' | b'\'' | b'"' | b'|' | b';' | b'&'))
}

fn push_colored(out: &mut String, text: &str, color: &str) {
    out.push_str(color);
    out.push_str(text);
    out.push_str(RESET);
}