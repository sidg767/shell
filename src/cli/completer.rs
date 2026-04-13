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
    fn complete(&self, line:&str,pos:usize,_ctx:&Context<' >)->rustyline::Result<(usize, Vec<Pair>)>{
        let (start,word)=extract_word(line,pos);
        let first_word=!line[..start].trim().contains(' ');
        let mut candidates=Vec::new();

        if first_word{
            get_builtin(word,&mut candidates);
            get_pathexec(word,&mut candidates);
        }
        else{
            get_fs(word,&mut candidates);
        }
        candidates.sort_unstable_by(|a,b| a.display.cmp(&b.display));
        Ok((start,candidates))
}
}
fn get_builtin(word:&   str,candidates:&mut Vec<Pair>){
    for &cmd in Builtin{
        if cmd.starts_with(word){
            candidates.push(Pair{
                display:cmd.to_owned(),
                replacement:cmd.to_owned(),
            });
        }
    }
}
fn get_pathexec(word:&str,candidates:&mut Vec<Pair>){
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