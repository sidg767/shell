use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use std::os::unix::fs::PermissionsExt;

const Builtin: &[&str] = &["cd", "echo", "pwd", "type", "exit"];

#[derive(Default)]
pub struct Completer;
impl Completer {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}
impl Completes for Completer{
    type Candidate=Pair;
    fn complete(&self, line:&str,pos:usize,_ctx:&Context<' >)->rustyline::Result<(usize, Vec<Pair>)>(
        let (start,word)=extract_word(line,pos);
        let first_word=!line[..start].trim().contains(' ');
        let mut candidates=Vec::new();

        if first_word{
            get_builtin(word,&mut candidates);
            get_pathexec(word,&mut candidates);
        }
        else{
            collect
        }

    )
}