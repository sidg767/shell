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
impl Shell {
    pub fn new() -> rustyline::Result<Self> {
        let helper = ShellHelper {
            completer:   ShellCompleter::new(),
            highlighter: ShellHighlighter::new(),
            hinter:      ShellHinter::new(),
            validator:   ShellValidator::new(),
        };

        let mut editor = Editor::new()?;
        editor.set_helper(Some(helper));
        editor.set_history_max_len(HISTORY_LIMIT)?;

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
        let pipeline: Vec<&str> = line.splitn(2, '|').collect();

        if pipeline.len() == 2 {
            self.run_pipeline(pipeline[0].trim(), pipeline[1].trim());
        } else {
            let (cmd, args) = parse_command(line);
            self.run_command(cmd, args);
        }
    }

    fn run_command(&self, cmd: &str, args: Vec<&str>) {
        match cmd {
            "exit" => std::process::exit(0),
            "cd"   => self.builtin_cd(args),
            "echo" => self.builtin_echo(args),
            "pwd"  => self.builtin_pwd(),
            "type" => self.builtin_type(args),
            _      => self.spawn_external(cmd, args, None, None),
        }
    }

    fn run_pipeline(&self, left: &str, right: &str) {
        let (lcmd, largs) = parse_command(left);
        let (rcmd, rargs) = parse_command(right);

        let mut left_child = match Command::new(lcmd)
            .args(&largs)
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(c)  => c,
            Err(e) => { eprintln!("{}: {}", lcmd, e); return; }
        };

        let stdin = match left_child.stdout.take() {
            Some(s) => Stdio::from(s),
            None    => { eprintln!("pipeline: failed to capture stdout"); return; }
        };

        let mut right_child = match Command::new(rcmd)
            .args(&rargs)
            .stdin(stdin)
            .spawn()
        {
            Ok(c)  => c,
            Err(e) => { eprintln!("{}: {}", rcmd, e); return; }
        };

        let _ = left_child.wait();
        let _ = right_child.wait();
    }
 fn spawn_external(
        &self,
        cmd: &str,
        args: Vec<&str>,
        stdin:  Option<Stdio>,
        stdout: Option<Stdio>,
    ) {
        let mut builder = Command::new(cmd);
        builder.args(&args);

        if let Some(s) = stdin  { builder.stdin(s);  }
        if let Some(s) = stdout { builder.stdout(s); }

        match builder.spawn() {
            Ok(mut child) => { let _ = child.wait(); }
            Err(e) => eprintln!("{}: {}", cmd, e),
        }
    }

    fn builtin_cd(&self, args: Vec<&str>) {
        let target = args.first()
            .map(|s| Path::new(s))
            .unwrap_or_else(|| Path::new("."));

        if let Err(e) = std::env::set_current_dir(target) {
            eprintln!("cd: {}", e);
        }
    }

    fn builtin_echo(&self, args: Vec<&str>) {
        println!("{}", args.join(" "));
    }

    fn builtin_pwd(&self) {
        match std::env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e)   => eprintln!("pwd: {}", e),
        }
    }

    fn builtin_type(&self, args: Vec<&str>) {
        for name in args {
            if ["cd","echo","pwd","type","exit"].contains(&name) {
                println!("{} is a shell builtin", name);
            } else if let Some(path) = find_in_path(name) {
                println!("{} is {}", name, path);
            } else {
                eprintln!("{}: not found", name);
            }
        }
    }
}

fn build_prompt() -> String {
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    format!("{} $ ", cwd)
}

fn parse_command(input: &str) -> (&str, Vec<&str>) {
    let mut parts = input.split_whitespace();
    let cmd  = parts.next().unwrap_or("");
    let args = parts.collect();
    (cmd, args)
}

fn find_in_path(name: &str) -> Option<String> {
    let path_var = std::env::var("PATH").ok()?;

    path_var.split(':')
        .map(|dir| format!("{}/{}", dir, name))
        .find(|full| Path::new(full).is_file())
}

pub fn start() -> rustyline::Result<()> {
    Shell::new()?.start()
}

   