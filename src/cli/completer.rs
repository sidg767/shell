use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use rustyline::hint::{Hinter, HistoryHinter};
use rustyline::highlight::Highlighter;
use rustyline::validate::Validator;
use rustyline::Helper;

const BUILTINS: &[&str] = &["cd", "echo", "pwd", "type", "exit"];
#[derive(Default)]
pub struct ShellCompleter {
    history_hinter: HistoryHinter,
}

impl ShellCompleter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            history_hinter: HistoryHinter {},
        }
    }
}

impl Helper for ShellCompleter {}

impl Hinter for ShellCompleter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        self.history_hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for ShellCompleter {}

impl Validator for ShellCompleter {}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) = extract_word(line, pos);
        let is_first_word = !line[..start].trim().contains(' ');
        let mut candidates: Vec<Pair> = Vec::new();

        if is_first_word {
            collect_builtins(word, &mut candidates);
            collect_path_executables(word, &mut candidates);
        } else {
            collect_filesystem(word, &mut candidates);
        }

        candidates.sort_unstable_by(|a, b| a.display.cmp(&b.display));
        candidates.dedup_by(|a, b| a.display == b.display);

        Ok((start, candidates))
    }
}

fn collect_builtins(word: &str, candidates: &mut Vec<Pair>) {
    for &builtin in BUILTINS {
        if builtin.starts_with(word) {
            candidates.push(Pair {
                display: builtin.to_owned(),
                replacement: builtin.to_owned(),
            });
        }
    }
}

fn collect_path_executables(word: &str, candidates: &mut Vec<Pair>) {
    let path_var = match std::env::var("PATH") {
        Ok(p) => p,
        Err(_) => return,
    };

    for dir in path_var.split(':') {
        let entries = match fs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => continue,
        };

        for entry in entries.flatten() {
            let name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };

            if !name.starts_with(word) {
                continue;
            }

            let Ok(metadata) = entry.metadata() else { continue };
            let is_executable = metadata.permissions().mode() & 0o111 != 0;

            if metadata.is_file() && is_executable {
                candidates.push(Pair {
                    display: name.clone(),
                    replacement: name,
                });
            }
        }
    }
}

fn collect_filesystem(word: &str, candidates: &mut Vec<Pair>) {
    let (search_dir, prefix) = match word.rfind('/') {
        Some(pos) => (&word[..=pos], &word[pos + 1..]),
        None => (".", word),
    };

    let entries = match fs::read_dir(search_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };

        if !name.starts_with(prefix) {
            continue;
        }

        let is_dir = entry.file_type()
            .map(|ft| ft.is_dir())
            .unwrap_or(false);

        let replacement = if search_dir == "." {
            if is_dir { format!("{}/", name) } else { name }
        } else {
            if is_dir { format!("{}{}/", search_dir, name) } else { format!("{}{}", search_dir, name) }
        };

        candidates.push(Pair {
            display: replacement.clone(),
            replacement,
        });
    }
}

fn extract_word(line: &str, pos: usize) -> (usize, &str) {
    let line = &line[..pos];
    let start = line.rfind(|c: char| c == ' ' || c == '|' || c == ';')
        .map(|i| i + 1)
        .unwrap_or(0);
    (start, &line[start..])
}